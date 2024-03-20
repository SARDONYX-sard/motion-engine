use super::{cpp_type_parser::parse_cpp_type, bitflags::generate_bitflags};
use crate::hkxcmd_parser::{ClassInfo, Enum, FlagValues, MemberInfo};
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use std::{borrow::Cow, collections::HashMap};

/// (C++ class name, C++ class info)
///
/// e.g. `(hkColor, ClassInfo)`
pub type ClassMap = IndexMap<String, ClassInfo>;

/// (`rust enum name`, `Rust enum name with lifeTime`)
///
/// e.g. `(HkColor, HkColor<'a>)`
pub type LifeTimeMap = HashMap<String, String>;

/// `C++ field name`, (`Rust tag name of enum`, `Rust type name`)
///
/// e.g. `referenceCount`, (`ReferenceCount`, `Primitive<i16>`)
pub type FieldMap<'a> = IndexMap<&'a String, (String, Cow<'a, str>)>;

/// Generate Rust XML Serialize/Deserialize structs code from C++ class info>
pub fn generate_code(
    cpp_class_name: &str,
    classes_map: &ClassMap,
    life_time_map: &LifeTimeMap,
) -> String {
    let mut rust_code = String::new();

    let class = classes_map.get(cpp_class_name).unwrap();
    let ClassInfo {
        signature,
        vtable,
        name: class_name,
        parent,
        size,
        version,
        ..
    } = class;

    let parent_info = parent
        .as_ref()
        .map(|(name, signature)| format!("\n/// -    parent: `{name}`/`0x{signature:x}`"))
        .unwrap_or_default();

    // ! The lifetime annotation of a structure cannot be made without first calculating whether or not the fields has a lifetime.
    let (rust_fields_code, fields) = generate_all_fields(class, classes_map, Some(life_time_map));

    // e.g. `HkColor<'a>`
    let rust_enum_name = class_name.to_case(Case::Pascal);
    let life_time = get_lifetime_from_fields(&fields);
    let rust_enum_name_with_life_time = format!("{rust_enum_name}{life_time}");

    // Because the manual deserializer cannot be implemented if there is nothing in the tag of the enum representing all fields in the C++ Class.
    // Derive `serde::Deserialize`.
    let derive_de = if fields.is_empty() {
        ", Deserialize"
    } else {
        ""
    };

    //? - Generate Struct
    rust_code.push_str(&format!(
        r#"//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `{class_name}`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `{class_name}`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: {size}
/// -    vtable: {vtable}{parent_info}
/// - signature: `0x{signature:x}`
/// -   version: {version}
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize{derive_de})]
#[serde(tag = "@name")]
pub enum {rust_enum_name_with_life_time} {{
{rust_fields_code}}}
"#
    ));

    // If there is no field, leave it to `derive`. Otherwise, implement manually.
    if !fields.is_empty() {
        let deserializer = generate_manual_tagged_de(&rust_enum_name_with_life_time, &fields);
        rust_code.push_str(&deserializer);
    }
    if !class.enums.is_empty() {
        for enum_info in &class.enums {
            let (enum_name, _enum_pair) = enum_info;

            if matches!(
                enum_name.as_str(),
                "FlagBits" | "FlagValues" | "Flags" | "HintFlags" | "RoleFlags" | "TransitionFlags"
            ) {
                rust_code.push_str(&generate_bitflags(enum_info));
            } else {
                rust_code.push_str(&generate_enums(enum_info));
            };
        }
    }
    rust_code
}

/// Return `"<'a>"` if there is at least one lifetime annotation in the information that has been changed from a C++ field type to a Rust type.
///
/// This exists to get the information of the lifetime annotation, because if the field has a lifetime, the lifetime annotation must be declared in advance in struct.
pub fn get_lifetime_from_fields(index_map: &FieldMap) -> &'static str {
    for (_field_name, (_, r_type)) in index_map {
        if r_type.find("'a").is_some() {
            return "<'a>";
        }
    }
    ""
}

/// Get the given string and generate a modified string with `'a` added
///
/// e.g. key `EventProperty` => `EventProperty<'a>` or `EventProperty` value from [`HashMap`]
fn get_type_with_lifetime<'a>(
    rust_type_key: &'a str,
    life_time_map: Option<&'a LifeTimeMap>,
) -> Option<String> {
    let type_with_life_time = life_time_map.map(|map: &LifeTimeMap| map.get(rust_type_key))??;
    Some(type_with_life_time.to_owned())
}

/// Assign lifetime generics to the passed array type according to the type found in [`HashMap`].
fn add_lifetime_to_array(rust_type: &str, life_time_map: Option<&LifeTimeMap>) -> String {
    // Extract the portion between '<' and '>'
    let start_index = rust_type.find('<').unwrap() + 1;
    let end_index = rust_type.rfind('>').unwrap();
    let inner = &rust_type[start_index..end_index];
    let inner = get_type_with_lifetime(inner, life_time_map).unwrap_or(inner.to_owned());

    // Concatenate the prefix of the original string with the generated string
    format!("{}{inner}>", &rust_type[..start_index])
}

/// Return (Rust code, Rust enum tags that C++ Class fields)
///
/// # Information
/// - The lifetime annotation of a structure cannot be made without first calculating whether or not the field has a lifetime.
/// - Flatten the C++ parent's inherited moves to the fields of the current class.
pub fn generate_all_fields<'a>(
    class: &'a ClassInfo,
    classes_map: &'a ClassMap,
    life_time_map: Option<&LifeTimeMap>,
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

/// Generates C++ fields to Rust enum
/// - Return `(generated code, IndexMap<"C++ field name", ("rust enum tag name", "rust type name")>)`
fn generate_fields<'a>(
    members: &'a [MemberInfo],
    life_time_map: Option<&LifeTimeMap>,
) -> (String, FieldMap<'a>) {
    let mut fields = IndexMap::new();
    let mut rust_code = String::new();

    for member in members {
        let MemberInfo {
            name: member_name,
            type_name,
            offset,
            flags,
            ..
        } = member;

        let mut skip_serializing_attr = String::new();
        if flags.contains(FlagValues::SERIALIZE_IGNORED) {
            skip_serializing_attr.push_str(", skip_serializing")
        }
        let flags = flags.human_readable();

        let (_, rust_type) = parse_cpp_type(type_name).unwrap();
        let rust_type =
            match rust_type.starts_with("HkArray") || rust_type.starts_with("SingleClass") {
                true => add_lifetime_to_array(&rust_type, life_time_map),
                false => get_type_with_lifetime(&rust_type, life_time_map)
                    .unwrap_or(rust_type.to_string())
                    .to_string(),
            };

        #[cfg(test)]
        if type_name.starts_with("flags") && type_name != "flags unknown" {
            tracing::debug!("\"{type_name}\" | ");
        };
        let type_name = type_name.replace("&lt;", "<").replace("&gt;", ">");

        // Enum tag name(If the first letter is a number, escape it with `_`.)
        let tag_name = member_name.to_case(Case::Pascal);
        let tag_name = match member_name.chars().next().map_or(false, |c| c.is_numeric()) {
            true => format!("_{tag_name}"),
            false => tag_name,
        };
        rust_code.push_str(&format!(
            r#"    /// # C++ Class Fields Info
    /// -   name:`"{member_name}"`
    /// -   type: `{type_name}`
    /// - offset: {offset}
    /// -  flags: `{flags}`
    #[serde(rename = "{member_name}"{skip_serializing_attr})]
    {tag_name}({rust_type}),
"#
        ));
        fields.insert(member_name, (tag_name, rust_type.into()));
    }

    (rust_code, fields)
}

/// tagged enum(Returns code that implements [`serde::Deserializer`], which branches processing for each value of
/// the `name` attribute of XML, which is each field of the C++ class.
fn generate_manual_tagged_de(rust_class_name_with_life_time: &str, fields: &FieldMap) -> String {
    let mut rust_code = String::new();

    let rust_class_name_with_life_time = rust_class_name_with_life_time.replace("'a", "'de");
    rust_code.push_str(&format!(
        r#"
// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {{
    {rust_class_name_with_life_time}, "@name",
"#
    ));

    for (member_name, (tag_name, rust_type)) in fields {
        let rust_type = rust_type.replace("'a", "'de");
        rust_code.push_str(&format!(
            r#"    ("{member_name}" => {tag_name}({rust_type})),
"#
        ));
    }

    rust_code.push_str("}\n");
    rust_code
}

/// Generate flags and C++ enum(If exists)
fn generate_enums(enum_info: &Enum) -> String {
    let mut rust_code = String::new();

    let (enum_name, enum_pair) = enum_info;
    let enum_name = enum_name.to_case(Case::Pascal);
    // Generate one enum template prefix
    rust_code.push_str(&format!(
        r#"
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum {enum_name} {{
"#
    ));

    // Generate tag & value pairs
    for (tag_name, enum_value) in enum_pair {
        let rust_enum_field_name = &tag_name.to_case(Case::Pascal);
        rust_code.push_str(&format!(
            r#"    #[serde(rename = "{tag_name}")]
    {rust_enum_field_name} = {enum_value},
"#
        ));
    }

    // Generate one enum template postfix
    rust_code.push_str("}\n");

    rust_code
}
