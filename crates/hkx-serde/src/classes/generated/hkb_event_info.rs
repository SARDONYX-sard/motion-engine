//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x5874eed4`
/// -   version: 0
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Flags {
    #[serde(rename = "FLAG_SILENT")]
    FlagSilent = 1,
    #[serde(rename = "FLAG_SYNC_POINT")]
    FlagSyncPoint = 2,
}
