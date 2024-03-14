//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMeshSection`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMeshSection<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkxVertexBuffer*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"indexBuffers"`
    /// -   type: `hkArray&lt;hkxIndexBuffer*&gt;`
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
    Material(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"userChannels"`
    /// -   type: `hkArray&lt;hkReferencedObject*&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userChannels")]
    UserChannels(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMeshSection<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Cow<'de, str>)),
    ("indexBuffers" => IndexBuffers(HkArrayRef<Cow<'de, str>>)),
    ("material" => Material(Cow<'de, str>)),
    ("userChannels" => UserChannels(HkArrayRef<Cow<'de, str>>)),
}
