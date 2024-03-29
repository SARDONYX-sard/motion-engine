//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBufferVertexBufferInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkMultipleVertexBufferVertexBufferInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xdafbe0e6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferVertexBufferInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"lockedVertices"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "lockedVertices", skip_serializing)]
    LockedVertices(Primitive<Cow<'a, str>>),
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
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("lockedVertices" => LockedVertices(Primitive<Cow<'de, str>>)),
    ("isLocked" => IsLocked(Primitive<bool>)),
}

impl ByteDeSerialize for HkMultipleVertexBufferVertexBufferInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
