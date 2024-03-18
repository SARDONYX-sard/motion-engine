//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMoppCodeCodeInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpMoppCodeCodeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xd8fdbb08`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMoppCodeCodeInfo {
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppCodeCodeInfo, "@name",
    ("offset" => Offset(Vector4<f32>)),
}
