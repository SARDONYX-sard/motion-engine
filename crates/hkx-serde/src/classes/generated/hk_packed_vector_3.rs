//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPackedVector3`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkPackedVector3`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x9c16df5b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPackedVector3 {
    /// # C++ Class Fields Info
    /// -   name:`"values"`
    /// -   type: `hkInt16[4]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "values")]
    Values(CStyleArray<[i16; 4]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPackedVector3, "@name",
    ("values" => Values(CStyleArray<[i16; 4]>)),
}
