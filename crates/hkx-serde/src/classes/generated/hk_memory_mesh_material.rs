//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshMaterial`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMemoryMeshMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkMeshMaterial`/`0xda8c7d7d`
/// - signature: `0x12156ee3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshMaterial<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"materialName"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialName")]
    MaterialName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"textures"`
    /// -   type: `hkArray&lt;hkMeshTexture*&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "textures")]
    Textures(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshMaterial<'de>, "@name",
    ("materialName" => MaterialName(Primitive<Cow<'de, str>>)),
    ("textures" => Textures(HkArrayRef<Cow<'de, str>>)),
}
