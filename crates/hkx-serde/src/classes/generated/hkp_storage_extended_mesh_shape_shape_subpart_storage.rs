//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShapeShapeSubpartStorage`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpStorageExtendedMeshShapeShapeSubpartStorage`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3f7d804c`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeShapeSubpartStorage {
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
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<struct hkpStorageExtendedMeshShapeMaterial>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(HkArrayNum<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeShapeSubpartStorage, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("materialIndices" => MaterialIndices(HkArrayNum<u8>)),
    ("materials" => Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>)),
    ("materialIndices16" => MaterialIndices16(HkArrayNum<u16>)),
}

impl ByteDeSerialize for HkpStorageExtendedMeshShapeShapeSubpartStorage {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
