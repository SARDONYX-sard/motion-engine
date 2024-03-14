//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShapeShapeSubpartStorage`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeShapeSubpartStorage {
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;struct hkpStorageExtendedMeshShapeMaterial&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(HkArrayRef<Primitive<u16>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeShapeSubpartStorage, "@name",
    ("materialIndices" => MaterialIndices(HkArrayRef<Primitive<u8>>)),
    ("materials" => Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>)),
    ("materialIndices16" => MaterialIndices16(HkArrayRef<Primitive<u16>>)),
}
