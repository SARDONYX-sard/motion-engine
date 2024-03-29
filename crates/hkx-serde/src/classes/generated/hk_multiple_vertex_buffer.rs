//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBuffer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkMultipleVertexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 324
/// -    vtable: true
/// -    parent: `hkMeshVertexBuffer`/`0x534b08c8`
/// - signature: `0xde3ab602`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBuffer<'a> {
    // C++ Parent class(`hkMeshVertexBuffer` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"vertexFormat"`
    /// -   type: `struct hkVertexFormat`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexFormat")]
    VertexFormat(SingleClass<HkVertexFormat>),
    /// # C++ Class Fields Info
    /// -   name:`"lockedElements"`
    /// -   type: `hkArray<struct hkMultipleVertexBufferLockedElement>`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockedElements")]
    LockedElements(HkArrayClass<HkMultipleVertexBufferLockedElement>),
    /// # C++ Class Fields Info
    /// -   name:`"lockedBuffer"`
    /// -   type: `struct hkMemoryMeshVertexBuffer*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockedBuffer")]
    LockedBuffer(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"elementInfos"`
    /// -   type: `hkArray<struct hkMultipleVertexBufferElementInfo>`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementInfos")]
    ElementInfos(HkArrayClass<HkMultipleVertexBufferElementInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexBufferInfos"`
    /// -   type: `hkArray<struct hkMultipleVertexBufferVertexBufferInfo>`
    /// - offset: 296
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBufferInfos")]
    VertexBufferInfos(HkArrayClass<HkMultipleVertexBufferVertexBufferInfo<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 308
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"isLocked"`
    /// -   type: `hkBool`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"updateCount"`
    /// -   type: `hkUint32`
    /// - offset: 316
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "updateCount")]
    UpdateCount(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"writeLock"`
    /// -   type: `hkBool`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "writeLock")]
    WriteLock(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isSharable"`
    /// -   type: `hkBool`
    /// - offset: 321
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isSharable")]
    IsSharable(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"constructionComplete"`
    /// -   type: `hkBool`
    /// - offset: 322
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constructionComplete")]
    ConstructionComplete(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBuffer<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertexFormat" => VertexFormat(SingleClass<HkVertexFormat>)),
    ("lockedElements" => LockedElements(HkArrayClass<HkMultipleVertexBufferLockedElement>)),
    ("lockedBuffer" => LockedBuffer(Primitive<Cow<'de, str>>)),
    ("elementInfos" => ElementInfos(HkArrayClass<HkMultipleVertexBufferElementInfo>)),
    ("vertexBufferInfos" => VertexBufferInfos(HkArrayClass<HkMultipleVertexBufferVertexBufferInfo<'de>>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("isLocked" => IsLocked(Primitive<bool>)),
    ("updateCount" => UpdateCount(Primitive<u32>)),
    ("writeLock" => WriteLock(Primitive<bool>)),
    ("isSharable" => IsSharable(Primitive<bool>)),
    ("constructionComplete" => ConstructionComplete(Primitive<bool>)),
}

impl ByteDeSerialize for HkMultipleVertexBuffer<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
