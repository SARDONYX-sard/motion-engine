//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGeneratorSyncInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbGeneratorSyncInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xa3c341f8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorSyncInfo {
    /// # C++ Class Fields Info
    /// -   name:`"syncPoints"`
    /// -   type: `struct hkbGeneratorSyncInfoSyncPoint[8]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncPoints")]
    SyncPoints(CStyleArrayClass<HkbGeneratorSyncInfoSyncPoint, 8>),
    /// # C++ Class Fields Info
    /// -   name:`"baseFrequency"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "baseFrequency")]
    BaseFrequency(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"playbackSpeed"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "playbackSpeed")]
    PlaybackSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numSyncPoints"`
    /// -   type: `hkInt8`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSyncPoints")]
    NumSyncPoints(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"isCyclic"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isCyclic")]
    IsCyclic(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isMirrored"`
    /// -   type: `hkBool`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isMirrored")]
    IsMirrored(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isAdditive"`
    /// -   type: `hkBool`
    /// - offset: 79
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isAdditive")]
    IsAdditive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorSyncInfo, "@name",
    ("syncPoints" => SyncPoints(CStyleArrayClass<HkbGeneratorSyncInfoSyncPoint, 8>)),
    ("baseFrequency" => BaseFrequency(Primitive<f32>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("playbackSpeed" => PlaybackSpeed(Primitive<f32>)),
    ("numSyncPoints" => NumSyncPoints(Primitive<i8>)),
    ("isCyclic" => IsCyclic(Primitive<bool>)),
    ("isMirrored" => IsMirrored(Primitive<bool>)),
    ("isAdditive" => IsAdditive(Primitive<bool>)),
}

impl ByteDeSerialize for HkbGeneratorSyncInfo {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
