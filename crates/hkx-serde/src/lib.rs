mod bytes;
mod classes;
mod error;
mod flag_values;
pub mod generators;
pub mod havok_types;
mod helpers;
mod hk_types;
mod parse_rpt;

use crate::{generators::rust::generate_code, parse_rpt::parse_class};
use convert_case::{Case, Casing};
use generators::rust::{generate_all_fields, generate_class_params, get_lifetime_from_map};
use indexmap::IndexMap;

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
        if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("xml")) {
            continue;
        }

        // Exclude some problematic classes that aren't needed
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        if matches!(
            file_name,
            "hkaiTraversalAnalysis"
                | "hkaiTraversalAnnotationLibraryAnnotation"
                | "hkaiTraversalAnnotationLibrary"
                | "hkaiTraversalAnalysisOutputSection"
                | "hkaiTraversalAnalysisOutput"
                | "hkbnpPhysicsInterface"
        ) {
            continue;
        }

        // Remove tailing version(e.g. _1)
        let file_stem = file_name
            .rsplit('_')
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .to_case(Case::Snake);

        let content = std::fs::read_to_string(path).unwrap();
        let (remain, class) = parse_class(&content).unwrap();

        tracing::debug!("remain = {:?}", remain);
        tracing::debug!("class = {:?}", class);

        mod_indexes.push(format!("mod {file_stem};\nuse {file_stem}::*;\n"));

        let class_name = class.name.clone();
        class_map.insert(class_name.clone(), class.clone());
    }
    mod_indexes.push("pub mod class_params;\n".into());
    std::fs::write(output_dir.join("mod.rs"), mod_indexes.join("\n")).unwrap();

    //? Create life time map (cpp class name, rust struct name with life time)
    let mut life_time_name_map = IndexMap::new();
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
        generate_classes()
    }
}
