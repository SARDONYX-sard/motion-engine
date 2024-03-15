//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedSubTrack1nInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSerializedSubTrack1nInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `hkpSerializedTrack1nInfo`/`0xf12d48d9`
/// - signature: `0x10155a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedSubTrack1NInfo {
    /// # C++ Parent class(`hkpSerializedTrack1nInfo`, parent: `None`) field Info
    /// -   name:`"sectors"`
    /// -   type: `hkArray&lt;hkpAgent1nSector*&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectors", default)]
    Sectors(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkpSerializedTrack1nInfo`, parent: `None`) field Info
    /// -   name:`"subTracks"`
    /// -   type: `hkArray&lt;hkpSerializedSubTrack1nInfo*&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subTracks", default)]
    SubTracks(HkArrayRef<Cow<'a, str>>),

    /// # C++ Class Fields Info
    /// -   name:`"sectorIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectorIndex", default)]
    SectorIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"offsetInSector"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetInSector", default)]
    OffsetInSector(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedSubTrack1NInfo, "@name",
    ("sectors" => Sectors(HkArrayRef<Cow<'de, str>>)),
    ("subTracks" => SubTracks(HkArrayRef<Cow<'de, str>>)),
    ("sectorIndex" => SectorIndex(Primitive<i32>)),
    ("offsetInSector" => OffsetInSector(Primitive<i32>)),
}
