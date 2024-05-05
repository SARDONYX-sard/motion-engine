use super::{aliases::ClassMap, one_class::all_fields::get_all_parents_info};
use crate::hkxcmd_parser::{hk_types::Type, parse_class, FlagValues};
use indexmap::IndexMap;
use std::{collections::HashMap, path::Path};
use topo_sort::TopoSort;

///  Get the aligned value.
///
/// <https://github.com/rust-lang/rust/blob/1.30.0/src/libcore/alloc.rs#L199-L219>
const fn align(offset: u32, align: u32) -> u32 {
    offset.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
}

/// Outputs json files of classes that take into account x86 and x86_64 offsets and size calculations.
pub fn generate_classes_json(output_dir: impl AsRef<Path>, rpt_dir: impl AsRef<Path>) {
    let output_dir = output_dir.as_ref();
    let rpt_dir = rpt_dir.as_ref();

    let mut class_map = IndexMap::new();
    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        let path = path.as_path();
        if !path.is_file() {
            continue;
        }

        let content = std::fs::read_to_string(path).unwrap();
        let (_remain, class) = parse_class(&content).unwrap();

        // The binary deserializer implementation process requires only four classes, but only parses them for parent class information.
        class_map.insert(class.name.clone(), class.clone());
    }

    generate_offset_info(output_dir, &class_map);
}

pub fn generate_offset_info(output_dir: impl AsRef<Path>, class_map: &ClassMap) {
    let ptr_size = 8;
    let mut topo_sort = TopoSort::with_capacity(class_map.len());

    for (cpp_class_name, class_info) in class_map {
        let mut deps: Vec<&String> = if let Some(ref parent) = class_info.parent {
            get_all_parents_info(&parent.0, class_map)
                .iter()
                .map(|info| &info.name)
                .collect()
        } else {
            Vec::new()
        };

        for member in &class_info.members {
            if let Some(ref class_ref) = member.class_ref {
                if member.hk_type == Type::Struct && !deps.contains(&class_ref) {
                    deps.push(class_ref);
                };
            }
        }

        tracing::debug!("{:?},{:#?}", &cpp_class_name, &deps);
        topo_sort.insert(cpp_class_name, deps)
    }

    match topo_sort.into_vec_nodes() {
        topo_sort::SortResults::Full(sorted_classes) => {
            let mut size_map = HashMap::new();
            let mut max_size_map = HashMap::new();

            // Get C++ class information from vec sorted by root order of dependencies and make json.
            for cpp_class_name in sorted_classes {
                tracing::debug!("cpp_class_name: {:?}", &cpp_class_name);
                let mut class_info = class_map[cpp_class_name].clone();
                let mut current_offset = 0;
                let mut max_member_size = 0; // Need this item for struct tailing alignment.

                // When inheriting from a parent class, the starting point is the size of the parent class to .
                if let Some(ref parent) = class_info.parent {
                    let parent_size = size_map[&parent.0];
                    let parent_max_size = max_size_map[&parent.0];

                    current_offset += parent_size;
                    max_member_size = parent_max_size;
                } else if class_info.members.is_empty() && class_info.vtable {
                    // Even if it is an empty field, in the case of a vtable, there is a ptr to the vtable, so this is taken into account.
                    current_offset = ptr_size;
                    max_member_size = ptr_size;
                };

                let mut prev_size = 0; // The previous field size is needed to align the next field.
                for (index, member) in &mut class_info.members.iter_mut().enumerate() {
                    let (mut current_member_size, mut current_max_size) =
                        if member.hk_type == Type::Struct {
                            let cpp_struct_name = member.class_ref.as_ref().unwrap();

                            (size_map[cpp_struct_name], max_size_map[cpp_struct_name])
                        } else {
                            (
                                member.type_size(&member.hk_type, ptr_size),
                                member.max_size(&member.hk_type, ptr_size),
                            )
                        };

                    if member.c_style_array_size > 0 {
                        current_member_size *= member.c_style_array_size as u32;
                    };

                    // Perform offset calculation for the current member.
                    if index != 0 {
                        current_offset += prev_size;
                        // The next field must be a multiple of the current size.
                        current_offset = align(current_offset, current_max_size);
                    };

                    // Alignment flags are enforced even in the first field.
                    if member.flags.contains(FlagValues::ALIGN16) {
                        current_offset = align(current_offset, 16);
                        if current_max_size < 16 {
                            current_max_size = 16;
                        }
                    } else if member.flags.contains(FlagValues::ALIGN8) {
                        current_offset = align(current_offset, 8);
                        if current_max_size < 8 {
                            current_max_size = 8;
                        }
                    };

                    member.offset_x86_64 = current_offset;
                    prev_size = current_member_size;

                    // Calculate for tailing alignment of struct with max member size.
                    if let Some(ref parent) = class_info.parent {
                        let parent_max_size = max_size_map[&parent.0];
                        if parent_max_size > current_max_size {
                            current_max_size = parent_max_size;
                        }
                    };

                    if current_max_size > max_member_size {
                        max_member_size = current_max_size;
                    };
                }

                max_size_map.insert(class_info.name.clone(), max_member_size);

                // Need tailing alignment for struct with max member size.
                let struct_size = align(current_offset + prev_size, max_member_size);
                class_info.size_x86_64 = struct_size;
                size_map.insert(class_info.name.clone(), struct_size);

                let output_dir = output_dir.as_ref();
                std::fs::create_dir_all(output_dir).unwrap();
                let mut path = output_dir.join(class_info.name.clone());
                path.set_extension("json");
                let json = serde_json::ser::to_string_pretty(&class_info).unwrap();
                std::fs::write(path, json).unwrap();
            }

            tracing::debug!("size_map = {:#?}", size_map);
            tracing::debug!("max_size_map = {:#?}", max_size_map);
        }
        topo_sort::SortResults::Partial(_) => todo!(),
    }
}

#[allow(unused)]
#[repr(C, align(16))]
struct Vector4 {
    v: [u32; 4],
}

#[allow(unused)]
#[repr(C)]
struct T {
    // 0
    t: u64, // 8bytes

    count: u16,     // 2bytes
    ref_count: u16, // 2bytes

    // 0x10
    t2: Vector4, // 16bytes
    // 0x20
    t3: u16, // 2bytes

    // 0x22
    _pad0: u64, // 8bytes

                // 0x30
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    pub fn should_generate_x86_64_json() {
        let _guard = hkx_serde_tracing::init_tracing(
            Some("should_x86_64_json"),
            false,
            tracing::Level::DEBUG,
        );

        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("classes");
        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");

        generate_classes_json(output_dir, rpt_dir)
    }
}
