//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkMemoryMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkMeshShape`/`0x9117d62e`
/// - signature: `0xb743a578`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshShape<'a> {
    // C++ Parent class(`hkMeshShape` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"sections"`
    /// -   type: `hkArray<struct hkMeshSectionCinfo>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sections")]
    Sections(HkArrayClass<HkMeshSectionCinfo<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshShape<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("sections" => Sections(HkArrayClass<HkMeshSectionCinfo<'de>>)),
    ("indices16" => Indices16(HkArrayNum<u16>)),
    ("indices32" => Indices32(HkArrayNum<u32>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkMemoryMeshShape<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
