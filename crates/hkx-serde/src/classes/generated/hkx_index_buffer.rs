//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxIndexBuffer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxIndexBuffer {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"indexType"`
    /// -   type: `enum IndexType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType")]
    IndexType(Primitive<IndexType>),
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(HkArrayNum<u32>),
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
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("indexType" => IndexType(Primitive<IndexType>)),
    ("indices16" => Indices16(HkArrayNum<u16>)),
    ("indices32" => Indices32(HkArrayNum<u32>)),
    ("vertexBaseOffset" => VertexBaseOffset(Primitive<u32>)),
    ("length" => Length(Primitive<u32>)),
}

impl ByteDeSerialize for HkxIndexBuffer {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
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
