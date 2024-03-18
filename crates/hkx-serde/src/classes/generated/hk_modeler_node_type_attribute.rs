//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkModelerNodeTypeAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkModelerNodeTypeAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x338c092f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkModelerNodeTypeAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum ModelerType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<ModelerType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkModelerNodeTypeAttribute, "@name",
    ("type" => Type(Primitive<ModelerType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelerType {
    #[serde(rename = "DEFAULT")]
    Default = 0,
    #[serde(rename = "LOCATOR")]
    Locator = 1,
}
