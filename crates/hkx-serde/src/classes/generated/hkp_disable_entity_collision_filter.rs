//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpDisableEntityCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpDisableEntityCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0xfac3351c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisableEntityCollisionFilter<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"disabledEntities"`
    /// -   type: `hkArray&lt;hkpEntity*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disabledEntities")]
    DisabledEntities(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisableEntityCollisionFilter<'de>, "@name",
    ("disabledEntities" => DisabledEntities(HkArrayRef<Cow<'de, str>>)),
}
