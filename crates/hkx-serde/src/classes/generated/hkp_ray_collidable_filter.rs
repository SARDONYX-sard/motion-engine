//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRayCollidableFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpRayCollidableFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: true
/// - signature: `0xe0708a00`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@name")]
pub enum HkpRayCollidableFilter {
}

impl ByteDeSerialize for HkpRayCollidableFilter {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
