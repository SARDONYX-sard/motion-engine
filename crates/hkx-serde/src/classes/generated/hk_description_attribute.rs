//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkDescriptionAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkDescriptionAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0xe9f9578a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkDescriptionAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"string"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "string")]
    String(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkDescriptionAttribute<'de>, "@name",
    ("string" => String(Primitive<Cow<'de, str>>)),
}
