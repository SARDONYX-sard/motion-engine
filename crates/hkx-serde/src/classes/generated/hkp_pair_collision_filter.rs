//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPairCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPairCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0x4abc140e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPairCollisionFilter<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"disabledPairs"`
    /// -   type: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "disabledPairs", skip_serializing)]
    DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType),
    /// # C++ Class Fields Info
    /// -   name:`"childFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childFilter")]
    ChildFilter(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPairCollisionFilter<'de>, "@name",
    ("disabledPairs" => DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType)),
    ("childFilter" => ChildFilter(Cow<'de, str>)),
}
