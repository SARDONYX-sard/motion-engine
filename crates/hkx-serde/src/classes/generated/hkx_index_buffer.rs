//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxIndexBuffer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxIndexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc12c8197`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxIndexBuffer {
    /// # C++ Class Fields Info
    /// -   name:`"indexType"`
    /// -   type: `enum IndexType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType")]
    IndexType(Primitive<IndexType>),
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexBaseOffset"`
    /// -   type: `hkUint32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBaseOffset")]
    VertexBaseOffset(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"length"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "length")]
    Length(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxIndexBuffer, "@name",
    ("indexType" => IndexType(Primitive<IndexType>)),
    ("indices16" => Indices16(HkArrayRef<Primitive<u16>>)),
    ("indices32" => Indices32(HkArrayRef<Primitive<u32>>)),
    ("vertexBaseOffset" => VertexBaseOffset(Primitive<u32>)),
    ("length" => Length(Primitive<u32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexType {
    #[serde(rename = "INDEX_TYPE_INVALID")]
    IndexTypeInvalid = 0,
    #[serde(rename = "INDEX_TYPE_TRI_LIST")]
    IndexTypeTriList = 1,
    #[serde(rename = "INDEX_TYPE_TRI_STRIP")]
    IndexTypeTriStrip = 2,
    #[serde(rename = "INDEX_TYPE_TRI_FAN")]
    IndexTypeTriFan = 3,
    #[serde(rename = "INDEX_TYPE_MAX_ID")]
    IndexTypeMaxId = 4,
}
