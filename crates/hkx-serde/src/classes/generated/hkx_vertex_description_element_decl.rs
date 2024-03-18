//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexDescriptionElementDecl`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxVertexDescriptionElementDecl`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x483a429b`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexDescriptionElementDecl {
    /// # C++ Class Fields Info
    /// -   name:`"byteOffset"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "byteOffset")]
    ByteOffset(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum DataType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<DataType>),
    /// # C++ Class Fields Info
    /// -   name:`"usage"`
    /// -   type: `enum DataUsage`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usage")]
    Usage(Primitive<DataUsage>),
    /// # C++ Class Fields Info
    /// -   name:`"byteStride"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "byteStride")]
    ByteStride(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numElements"`
    /// -   type: `hkUint8`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElements")]
    NumElements(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescriptionElementDecl, "@name",
    ("byteOffset" => ByteOffset(Primitive<u32>)),
    ("type" => Type(Primitive<DataType>)),
    ("usage" => Usage(Primitive<DataUsage>)),
    ("byteStride" => ByteStride(Primitive<u32>)),
    ("numElements" => NumElements(Primitive<u8>)),
}
