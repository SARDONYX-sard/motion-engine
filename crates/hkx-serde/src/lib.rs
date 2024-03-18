mod bytes;
mod classes;
mod error;
pub mod flag_values;
pub mod generators;
pub mod havok_types;
mod helpers;
mod hk_types;
mod parse_rpt;

use crate::generators::{
    generate_all_fields, generate_class_params, generate_code, get_lifetime_from_map,
};
use crate::parse_rpt::parse_class;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use std::collections::HashMap;

pub fn generate_classes() {
    let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("classes")
        .join("generated");
    std::fs::create_dir_all(&output_dir).unwrap();

    let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("hkxcmd_help")
        .join("rpt");

    let mut class_map = IndexMap::new();
    let mut mod_indexes = Vec::new();
    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        let path = path.as_path();
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("xml")) {
            continue;
        }

        // Exclude some problematic classes that aren't needed
        let file_name = path.file_stem().unwrap().to_str().unwrap();

        // Remove tailing version(e.g. _1)
        let file = file_name.rsplit('_').collect::<Vec<_>>();
        let file_name = *file.last().unwrap();

        if matches!(
            file_name,
            "hkaiTraversalAnalysis"
            | "hkaiTraversalAnalysisOutput"
            | "hkaiTraversalAnalysisOutputSection"
            | "hkaiTraversalAnnotationLibrary"
            | "hkaiTraversalAnnotationLibraryAnnotation"
            | "hkbnpPhysicsInterface"
            // Classes that currently error out
            | "hkClassMember" // duplicate enum error DeprecatedFlagValues
            | "hkColor" // ExtendedColors is duplicate enum error
            | "hkpAgent1nSector" // Unsupported stack size [u8;460]
            | "hkpConstraintData"// duplicate enum error ConstraintType
            | "hkpSerializedAgentNnEntry"  // Unsupported stack size [u8;160] nnEntryData
            | "hkpWheelConstraintDataAtoms" // duplicate enum error Axis
            // dependencies above
            | "hkClass" //HkClassMember
            | "hkMonitorStreamColorTable" // hkMonitorStreamColorTableColorPair
            | "hkMonitorStreamColorTableColorPair" // ExtendedColors
            | "hkpWheelConstraintData"
        ) {
            continue;
        }

        let content = std::fs::read_to_string(path).unwrap();
        let (remain, class) = parse_class(&content).unwrap();
        tracing::debug!("remain = {remain:?}\nclass = {class:?}");

        let file_stem = file_name.to_case(Case::Snake);
        mod_indexes.push(format!("mod {file_stem};\npub use {file_stem}::*;\n"));

        class_map.insert(class.name.clone(), class.clone());
    }
    mod_indexes.push("pub mod class_params;\n".into());
    std::fs::write(output_dir.join("mod.rs"), mod_indexes.join("\n")).unwrap();

    //? Create life time map (cpp class name, rust struct name with life time)
    let mut life_time_name_map = HashMap::new();
    for (_sig, class) in class_map.clone().into_iter() {
        let (_rust_fields_code, fields) = generate_all_fields(&class, &class_map, None);
        // IndexMap<"C++ field name", ("rust enum tag name", "rust type name")>
        let life_time = get_lifetime_from_map(&fields);
        let rust_struct_name = class.name.to_case(Case::Pascal);
        life_time_name_map.insert(
            rust_struct_name.clone(),
            format!("{rust_struct_name}{life_time}"),
        );
    }

    // I was able to detect the lifetime of `Cow<'a, str>` etc. in the first one, but not the lifetime of the newly attached structure in the case of structures with structure as a field.
    // Therefore, we will use the HashMap created in the first step to detect further nested lifetimes.
    for (_sig, class) in class_map.clone().into_iter() {
        let (_rust_fields_code, fields) =
            generate_all_fields(&class, &class_map, Some(&life_time_name_map));
        let life_time = get_lifetime_from_map(&fields);
        let rust_struct_name = class.name.to_case(Case::Pascal);
        life_time_name_map.insert(
            rust_struct_name.clone(),
            format!("{rust_struct_name}{life_time}"),
        );
    }

    for (_sig, class) in class_map.clone().into_iter() {
        let rust_file = output_dir.join(format!("{}.rs", class.name.to_case(Case::Snake)));
        let rust_code = generate_code(&class.name, class_map.clone(), life_time_name_map.clone());
        std::fs::write(rust_file, rust_code).unwrap();
    }

    let class_params = generate_class_params(class_map, life_time_name_map.clone());
    std::fs::write(output_dir.join("class_params.rs"), class_params).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_generate_classes() {
        let _guard =
            helpers::tracing::init_tracing(Some("should_generate_classes"), tracing::Level::DEBUG);
        generate_classes()
    }
}
