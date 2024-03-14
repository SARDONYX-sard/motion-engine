//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbSetLocalTimeOfClipGeneratorCommand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSetLocalTimeOfClipGeneratorCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xfab12b45`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetLocalTimeOfClipGeneratorCommand {
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeId"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeId")]
    NodeId(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetLocalTimeOfClipGeneratorCommand, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("nodeId" => NodeId(Primitive<i16>)),
}
