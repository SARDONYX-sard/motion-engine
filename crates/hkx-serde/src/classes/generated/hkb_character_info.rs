//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbCharacterInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacterInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd9709ff2`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterInfo {
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `enum Event`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(Event),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterInfo, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("event" => Event(Event)),
    ("padding" => Padding(Primitive<i32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "REMOVED_FROM_WORLD")]
    RemovedFromWorld = 0,
    #[serde(rename = "SHOWN")]
    Shown = 1,
    #[serde(rename = "HIDDEN")]
    Hidden = 2,
}
