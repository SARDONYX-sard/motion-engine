use super::cpp_type_parser::parse_cpp_type;
use crate::{
    flag_values::FlagValues,
    parse_rpt::{ClassInfo, MemberInfo},
};
use convert_case::{Case, Casing};
use indexmap::IndexMap;

pub fn get_life_time(vec: &Vec<MemberInfo>) -> &'static str {
    for m in vec {
        let (_input, ty) = parse_cpp_type(&m.type_name).unwrap();
        if ty.as_ref().contains("'a") {
            return "<'a>";
        }
    }
    ""
}

pub fn generate_code(signature: u32, classes_map: IndexMap<u32, ClassInfo>) -> String {
    let mut rust_code = String::new();

    let class = classes_map.get(&signature).unwrap();
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

    let (parent_name, parent_signature) = parent.clone().unwrap_or(("None".into(), 0));

    let rust_enum_class_name = class_name.to_case(Case::Pascal);
    let life_time = get_life_time(members);

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
/// -    vtable: {vtable}
/// -    parent: `{parent_name}`/`0x{parent_signature:x}`
/// - signature: `0x{signature:x}`
/// -   version: {version}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum {rust_enum_class_name}{life_time} {{
"#
    ));

    //? - C++ class fields
    let mut field = IndexMap::new();
    for member in &class.members {
        let MemberInfo {
            name: member_name,
            type_name,
            offset,
            flags,
            ..
        } = &member;

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
        field.insert(member_name, (tag_name, rust_type));
    }
    rust_code.push_str("}\n");

    //? - Impl Deserialization
    let life_time = get_life_time(members).replace("'a", "'de");
    rust_code.push_str(&format!(
        r#"
// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {{
    {rust_enum_class_name}{life_time}, "@name",
"#
    ));
    for (member_name, (tag_name, rust_type)) in field {
        let rust_type = rust_type.replace("'a", "'de");
        rust_code.push_str(&format!(
            r#"    ("{member_name}" => {tag_name}({rust_type})),
"#
        ));
    }
    rust_code.push_str("}\n");

    //? - Enum definitions(If exists)
    for (enum_name, enum_info) in &class.enums {
        rust_code.push_str(&format!(
            r#"
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum {enum_name} {{
"#
        ));

        for (tag_name, enum_value) in enum_info {
            let rust_enum_field_name = &tag_name.to_case(Case::Pascal);
            rust_code.push_str(&format!(
                r#"    #[serde(rename = "{tag_name}")]
    {rust_enum_field_name} = {enum_value},
"#
            ));
        }

        rust_code.push_str("}\n");
    }

    rust_code
}
