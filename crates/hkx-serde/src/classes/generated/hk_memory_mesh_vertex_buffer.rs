//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMemoryMeshVertexBuffer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMemoryMeshVertexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 424
/// -    vtable: true
/// -    parent: `hkMeshVertexBuffer`/`0x534b08c8`
/// - signature: `0xa2e50753`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshVertexBuffer {
    /// # C++ Class Fields Info
    /// -   name:`"format"`
    /// -   type: `struct hkVertexFormat`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "format")]
    Format(HkVertexFormat),
    /// # C++ Class Fields Info
    /// -   name:`"elementOffsets"`
    /// -   type: `hkInt32[32]`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementOffsets")]
    ElementOffsets([Primitive<i32>; 32]),
    /// # C++ Class Fields Info
    /// -   name:`"memory"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 396
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memory")]
    Memory(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexStride"`
    /// -   type: `hkInt32`
    /// - offset: 408
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStride")]
    VertexStride(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"locked"`
    /// -   type: `hkBool`
    /// - offset: 412
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "locked")]
    Locked(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 416
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"isBigEndian"`
    /// -   type: `hkBool`
    /// - offset: 420
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isBigEndian")]
    IsBigEndian(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isSharable"`
    /// -   type: `hkBool`
    /// - offset: 421
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isSharable")]
    IsSharable(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshVertexBuffer, "@name",
    ("format" => Format(HkVertexFormat)),
    ("elementOffsets" => ElementOffsets([Primitive<i32>; 32])),
    ("memory" => Memory(HkArrayRef<Primitive<u8>>)),
    ("vertexStride" => VertexStride(Primitive<i32>)),
    ("locked" => Locked(Primitive<bool>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("isBigEndian" => IsBigEndian(Primitive<bool>)),
    ("isSharable" => IsSharable(Primitive<bool>)),
}
