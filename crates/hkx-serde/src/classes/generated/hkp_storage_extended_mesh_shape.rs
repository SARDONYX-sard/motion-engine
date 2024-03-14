//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpStorageExtendedMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 272
/// -    vtable: true
/// -    parent: `hkpExtendedMeshShape`/`0x177114a2`
/// - signature: `0xb469efbc`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"meshstorage"`
    /// -   type: `hkArray&lt;hkpStorageExtendedMeshShapeMeshSubpartStorage*&gt;`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "meshstorage")]
    Meshstorage(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"shapestorage"`
    /// -   type: `hkArray&lt;hkpStorageExtendedMeshShapeShapeSubpartStorage*&gt;`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapestorage")]
    Shapestorage(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShape<'de>, "@name",
    ("meshstorage" => Meshstorage(HkArrayRef<Cow<'de, str>>)),
    ("shapestorage" => Shapestorage(HkArrayRef<Cow<'de, str>>)),
}
