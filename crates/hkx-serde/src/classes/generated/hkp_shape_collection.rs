//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpShapeCollection`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpShapeCollection`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0xe8c3991d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpShapeCollection {
    /// # C++ Class Fields Info
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disableWelding")]
    DisableWelding(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collectionType")]
    CollectionType(Primitive<CollectionType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpShapeCollection, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectionType {
    #[serde(rename = "COLLECTION_LIST")]
    CollectionList = 0,
    #[serde(rename = "COLLECTION_EXTENDED_MESH")]
    CollectionExtendedMesh = 1,
    #[serde(rename = "COLLECTION_TRISAMPLED_HEIGHTFIELD")]
    CollectionTrisampledHeightfield = 2,
    #[serde(rename = "COLLECTION_USER")]
    CollectionUser = 3,
    #[serde(rename = "COLLECTION_SIMPLE_MESH")]
    CollectionSimpleMesh = 4,
    #[serde(rename = "COLLECTION_MESH_SHAPE")]
    CollectionMeshShape = 5,
    #[serde(rename = "COLLECTION_COMPRESSED_MESH")]
    CollectionCompressedMesh = 6,
    #[serde(rename = "COLLECTION_MAX")]
    CollectionMax = 7,
}
