//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbModifierGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbModifierGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x1f81fae6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbModifierGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"modifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modifier")]
    Modifier(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generator")]
    Generator(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbModifierGenerator<'de>, "@name",
    ("modifier" => Modifier(Cow<'de, str>)),
    ("generator" => Generator(Cow<'de, str>)),
}
