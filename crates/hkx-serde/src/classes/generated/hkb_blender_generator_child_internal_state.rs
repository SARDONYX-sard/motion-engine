//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlenderGeneratorChildInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbBlenderGeneratorChildInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// - signature: `0xff7327c0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlenderGeneratorChildInternalState {
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isActive")]
    IsActive(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"syncNextFrame"`
    /// -   type: `hkBool`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncNextFrame")]
    SyncNextFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorChildInternalState, "@name",
    ("isActive" => IsActive(Primitive<bool>)),
    ("syncNextFrame" => SyncNextFrame(Primitive<bool>)),
}

impl ByteDeSerialize for HkbBlenderGeneratorChildInternalState {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
