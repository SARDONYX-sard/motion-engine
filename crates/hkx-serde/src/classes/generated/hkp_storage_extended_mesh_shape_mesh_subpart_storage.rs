//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShapeMeshSubpartStorage`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpStorageExtendedMeshShapeMeshSubpartStorage`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 104
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5aad4de6`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeMeshSubpartStorage<'a> {
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
    /// -   name:`"vertices"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices8"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices8")]
    Indices8(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<struct hkpStorageExtendedMeshShapeMaterial>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>),
    /// # C++ Class Fields Info
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray<struct hkpNamedMeshMaterial>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedMaterials")]
    NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(HkArrayNum<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeMeshSubpartStorage<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertices" => Vertices(HkArrayVector<Vector4<f32>>)),
    ("indices8" => Indices8(HkArrayNum<u8>)),
    ("indices16" => Indices16(HkArrayNum<u16>)),
    ("indices32" => Indices32(HkArrayNum<u32>)),
    ("materialIndices" => MaterialIndices(HkArrayNum<u8>)),
    ("materials" => Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>)),
    ("namedMaterials" => NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'de>>)),
    ("materialIndices16" => MaterialIndices16(HkArrayNum<u16>)),
}
