//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBufferElementInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMultipleVertexBufferElementInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// - signature: `0x4731fb1b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferElementInfo {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBufferIndex")]
    VertexBufferIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"elementIndex"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementIndex")]
    ElementIndex(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferElementInfo, "@name",
    ("vertexBufferIndex" => VertexBufferIndex(Primitive<u8>)),
    ("elementIndex" => ElementIndex(Primitive<u8>)),
}
