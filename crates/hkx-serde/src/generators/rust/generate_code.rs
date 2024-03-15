use super::cpp_type_parser::parse_cpp_type;
use crate::{
    flag_values::FlagValues,
    parse_rpt::{ClassInfo, Enum, MemberInfo},
};
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use std::borrow::Cow;

pub fn get_life_time(vec: &Vec<MemberInfo>) -> &'static str {
    for m in vec {
        let (_input, ty) = parse_cpp_type(&m.type_name).unwrap();
        if ty.as_ref().contains("'a") {
            return "<'a>";
        }
    }
    ""
}

pub fn generate_code(cpp_class_name: &str, classes_map: IndexMap<String, ClassInfo>) -> String {
    let mut rust_code = String::new();

    let class = classes_map.get(cpp_class_name).unwrap();
    let ClassInfo {
        signature,
        vtable,
        name: class_name,
        parent,
        size,
        members,
        version,
        ..
    } = class;

    let parent_info = parent
        .as_ref()
        .map(|(name, signature)| format!("\n/// -    parent: `{name}`/`0x{signature:x}`"))
        .unwrap_or_default();

    let rust_enum_class_name = class_name.to_case(Case::Pascal);

    // ! The lifetime annotation of a structure cannot be made without first calculating whether or not the field has a lifetime.
    //? - Flatten the C++ parent's inherited moves to the fields of the current class.
    let mut all_fields_code = String::new();
    let mut fields = IndexMap::new();
    let mut current_parent_class_name = parent
        .as_ref()
        .map(|(name, _sig)| name.clone())
        .unwrap_or_default();
    while let Some(parent_class) = classes_map.get(&current_parent_class_name) {
        let (fields_code, field) = generate_fields(&parent_class.members);
        fields.extend(field);

        let parent_name = &parent_class.name;
        let parent_of_parent = &parent_class
            .parent
            .as_ref()
            .map(|(name, _)| name.as_str())
            .unwrap_or("None");

        let fields_code = match fields_code.is_empty() {
            true => format!("    // `{parent_name}`(Parent class) has no fields\n"),
            false => fields_code.replace(
                "C++ Class Fields Info",
                &format!(
                    "C++ Parent class(`{parent_name}`, parent: `{parent_of_parent}`) field Info"
                ),
            ),
        };
        all_fields_code.push_str(&format!("{fields_code}\n",));
        if let Some((parent_name, _parent_signature)) = parent_class.parent.clone() {
            current_parent_class_name = parent_name;
        } else {
            break; // No more parent to process
        }
    }
    //? - Current C++ class fields
    let (fields_code, field) = generate_fields(&class.members);
    fields.extend(field);
    all_fields_code.push_str(&format!("{fields_code}}}\n"));

    let life_time = if all_fields_code.contains("'a") {
        "<'a>"
    } else {
        ""
    };
    //? - Generate Struct
    rust_code.push_str(&format!(
        r#"//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `{class_name}`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{{Deserialize, Serialize}};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum {rust_enum_class_name}{life_time} {{
"#
    ));

    rust_code.push_str(&all_fields_code);

    //? - Impl Deserialization
    let life_time = get_life_time(members).replace("'a", "'de");
    rust_code.push_str(&format!(
        r#"
// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {{
    {rust_enum_class_name}{life_time}, "@name",
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

    rust_code.push_str(&generate_enums(&class.enums));

    rust_code
}

/// Generates C++ fields to Rust enum
///
/// Return `(generated code, IndexMap<"C++ field name", ("rust enum tag name", "rust type name")>)`
fn generate_fields(members: &[MemberInfo]) -> (String, IndexMap<&String, (String, Cow<'_, str>)>) {
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

        // Enum tag name
        let tag_name = member_name.to_case(Case::Pascal);
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
        fields.insert(member_name, (tag_name, rust_type));
    }

    (rust_code, fields)
}

/// Generate Enum definitions rust code(If exists)
fn generate_enums(enums: &[Enum]) -> String {
    let mut rust_code = String::new();

    for (enum_name, enum_info) in enums {
        // Generate one enum template prefix
        rust_code.push_str(&format!(
            r#"
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum {enum_name} {{
"#
        ));

        // Generate tag names of enum
        for (tag_name, enum_value) in enum_info {
            let rust_enum_field_name = &tag_name.to_case(Case::Pascal);
            rust_code.push_str(&format!(
                r#"    #[serde(rename = "{tag_name}")]
    {rust_enum_field_name} = {enum_value},
"#
            ));
        }

        // Generate one enum template postfix
        rust_code.push_str("}\n");
    }

    rust_code
}
