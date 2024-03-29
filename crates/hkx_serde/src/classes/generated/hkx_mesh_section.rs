//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMeshSection`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkxMeshSection`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xe2286cf8`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMeshSection<'a> {
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
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkxVertexBuffer*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"indexBuffers"`
    /// -   type: `hkArray<hkxIndexBuffer*>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexBuffers")]
    IndexBuffers(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `struct hkxMaterial*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"userChannels"`
    /// -   type: `hkArray<hkReferencedObject*>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userChannels")]
    UserChannels(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMeshSection<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("indexBuffers" => IndexBuffers(HkArrayRef<Cow<'de, str>>)),
    ("material" => Material(Primitive<Cow<'de, str>>)),
    ("userChannels" => UserChannels(HkArrayRef<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkxMeshSection<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
