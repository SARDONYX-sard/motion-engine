//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAttributeModifierAssignment`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbAttributeModifierAssignment`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x48b8ad52`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAttributeModifierAssignment {
    /// # C++ Class Fields Info
    /// -   name:`"attributeIndex"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeIndex")]
    AttributeIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"attributeValue"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeValue")]
    AttributeValue(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttributeModifierAssignment, "@name",
    ("attributeIndex" => AttributeIndex(Primitive<i32>)),
    ("attributeValue" => AttributeValue(Primitive<f32>)),
}
