use crate::{
    generators::aliases::{ClassMap, FieldMap, LifeTimeMap},
    hkxcmd_parser::{ClassInfo, MemberInfo},
};
use indexmap::IndexMap;

/// Return (Rust code, Rust enum tags that C++ Class fields)
///
/// # Information
/// - The lifetime annotation of a structure cannot be made without first calculating whether or not the field has a lifetime.
/// - Flatten the C++ parent's inherited moves to the fields of the current class.
pub fn generate_all_fields<'a>(
    class: &'a ClassInfo,
    classes_map: &'a ClassMap,
    life_time_map: Option<&LifeTimeMap>,
    generate_fields: fn(&'a [MemberInfo], Option<&LifeTimeMap>) -> (String, FieldMap<'a>),
) -> (String, FieldMap<'a>) {
    let mut all_fields_code = String::new();
    let mut fields = IndexMap::new();
    let mut current_parent_class_name = class
        .parent
        .as_ref()
        .map(|(name, _sig)| name.clone())
        .unwrap_or_default();

    //? - All parent class fields of Current C++ class
    while let Some(parent_class) = classes_map.get(&current_parent_class_name) {
        let (fields_code, field) = generate_fields(&parent_class.members, life_time_map);
        fields.extend(field);

        let parent_name = &parent_class.name;
        let parent_of_parent = &parent_class
            .parent
            .as_ref()
            .map(|(name, _)| name.as_str())
            .unwrap_or("None");

        let fields_code = match fields_code.is_empty() {
            true => format!("    // C++ Parent class(`{parent_name}` => parent: `{parent_of_parent}`) has no fields\n    //"),
            false => {
                let parent_info = format!(
                    "C++ Parent class(`{parent_name}` => parent: `{parent_of_parent}`) field Info"
                );
                fields_code.replace("C++ Class Fields Info", &parent_info)
            }
        };
        all_fields_code.push_str(&fields_code);
        all_fields_code.push('\n');

        if let Some((parent_name, _parent_signature)) = &parent_class.parent {
            current_parent_class_name = parent_name.clone();
        } else {
            break; // No more parent to process
        }
    }

    //? - Current C++ class fields
    let (fields_code, field) = generate_fields(&class.members, life_time_map);
    fields.extend(field);
    all_fields_code.push_str(&fields_code);

    (all_fields_code, fields)
}
