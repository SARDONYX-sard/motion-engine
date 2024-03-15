//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedTrack1nInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedTrack1NInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sectors"`
    /// -   type: `hkArray&lt;hkpAgent1nSector*&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectors", default)]
    Sectors(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"subTracks"`
    /// -   type: `hkArray&lt;hkpSerializedSubTrack1nInfo*&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subTracks", default)]
    SubTracks(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedTrack1NInfo<'de>, "@name",
    ("sectors" => Sectors(HkArrayRef<Cow<'de, str>>)),
    ("subTracks" => SubTracks(HkArrayRef<Cow<'de, str>>)),
}
