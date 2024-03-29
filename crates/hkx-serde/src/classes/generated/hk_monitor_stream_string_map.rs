//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMonitorStreamStringMap`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkMonitorStreamStringMap`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xc4d3a8b4`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamStringMap<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"map"`
    /// -   type: `hkArray<struct hkMonitorStreamStringMapStringMap>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "map")]
    Map(HkArrayClass<HkMonitorStreamStringMapStringMap<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamStringMap<'de>, "@name",
    ("map" => Map(HkArrayClass<HkMonitorStreamStringMapStringMap<'de>>)),
}

impl ByteDeSerialize for HkMonitorStreamStringMap<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
