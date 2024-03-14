//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShapeMeshSubpartStorage`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeMeshSubpartStorage {
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices8"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices8")]
    Indices8(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;struct hkpStorageExtendedMeshShapeMaterial&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>),
    /// # C++ Class Fields Info
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray&lt;struct hkpNamedMeshMaterial&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedMaterials")]
    NamedMaterials(HkArrayClass<HkpNamedMeshMaterial>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(HkArrayRef<Primitive<u16>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeMeshSubpartStorage, "@name",
    ("vertices" => Vertices(HkArrayVector<Vector4<f32>>)),
    ("indices8" => Indices8(HkArrayRef<Primitive<u8>>)),
    ("indices16" => Indices16(HkArrayRef<Primitive<u16>>)),
    ("indices32" => Indices32(HkArrayRef<Primitive<u32>>)),
    ("materialIndices" => MaterialIndices(HkArrayRef<Primitive<u8>>)),
    ("materials" => Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>)),
    ("namedMaterials" => NamedMaterials(HkArrayClass<HkpNamedMeshMaterial>)),
    ("materialIndices16" => MaterialIndices16(HkArrayRef<Primitive<u16>>)),
}
