//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMultipleVertexBufferVertexBufferInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMultipleVertexBufferVertexBufferInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xdafbe0e6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferVertexBufferInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"lockedVertices"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "lockedVertices", skip_serializing)]
    LockedVertices(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"isLocked"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferVertexBufferInfo<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Cow<'de, str>)),
    ("lockedVertices" => LockedVertices(Cow<'de, str>)),
    ("isLocked" => IsLocked(Primitive<bool>)),
}
