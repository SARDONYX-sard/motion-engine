//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexDescriptionElementDecl`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexDescriptionElementDecl {
    /// # C++ Class Fields Info
    /// -   name:`"byteOffset"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "byteOffset", default)]
    ByteOffset(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum DataType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<DataType>),
    /// # C++ Class Fields Info
    /// -   name:`"usage"`
    /// -   type: `enum DataUsage`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usage", default)]
    Usage(Primitive<DataUsage>),
    /// # C++ Class Fields Info
    /// -   name:`"byteStride"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "byteStride", default)]
    ByteStride(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numElements"`
    /// -   type: `hkUint8`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElements", default)]
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
