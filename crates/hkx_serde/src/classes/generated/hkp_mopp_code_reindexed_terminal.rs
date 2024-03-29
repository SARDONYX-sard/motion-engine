//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMoppCodeReindexedTerminal`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpMoppCodeReindexedTerminal`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x6ed8ac06`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMoppCodeReindexedTerminal {
    /// # C++ Class Fields Info
    /// -   name:`"origShapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "origShapeKey")]
    OrigShapeKey(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"reindexedShapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reindexedShapeKey")]
    ReindexedShapeKey(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppCodeReindexedTerminal, "@name",
    ("origShapeKey" => OrigShapeKey(Primitive<u32>)),
    ("reindexedShapeKey" => ReindexedShapeKey(Primitive<u32>)),
}

impl ByteDeSerialize for HkpMoppCodeReindexedTerminal {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
