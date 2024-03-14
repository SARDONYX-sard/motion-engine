//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpPhysicsData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPhysicsData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc2a461e4`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPhysicsData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"worldCinfo"`
    /// -   type: `struct hkpWorldCinfo*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldCinfo")]
    WorldCinfo(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"systems"`
    /// -   type: `hkArray&lt;hkpPhysicsSystem*&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "systems")]
    Systems(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhysicsData<'de>, "@name",
    ("worldCinfo" => WorldCinfo(Cow<'de, str>)),
    ("systems" => Systems(HkArrayRef<Cow<'de, str>>)),
}
