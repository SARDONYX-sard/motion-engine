//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedTrack1nInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpSerializedTrack1nInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0xf12d48d9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedTrack1NInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sectors"`
    /// -   type: `hkArray<hkpAgent1nSector*>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectors")]
    Sectors(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"subTracks"`
    /// -   type: `hkArray<hkpSerializedSubTrack1nInfo*>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subTracks")]
    SubTracks(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedTrack1NInfo<'de>, "@name",
    ("sectors" => Sectors(HkArrayRef<Cow<'de, str>>)),
    ("subTracks" => SubTracks(HkArrayRef<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkpSerializedTrack1NInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
