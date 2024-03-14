//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpRejectChassisListener`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpRejectChassisListener`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc4fa16c9`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRejectChassisListener<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"chassis"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "chassis", skip_serializing)]
    Chassis(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRejectChassisListener<'de>, "@name",
    ("chassis" => Chassis(Cow<'de, str>)),
}
