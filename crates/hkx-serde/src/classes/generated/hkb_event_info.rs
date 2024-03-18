//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbEventInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0x5874eed4`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventInfo {
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags Flags`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<Flags>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventInfo, "@name",
    ("flags" => Flags(Primitive<Flags>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Flags {
    #[serde(rename = "FLAG_SILENT")]
    FlagSilent = 1,
    #[serde(rename = "FLAG_SYNC_POINT")]
    FlagSyncPoint = 2,
}
