//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSModifyOnceModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSModifyOnceModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x1e20a97a`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsModifyOnceModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pOnActivateModifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pOnActivateModifier")]
    POnActivateModifier(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"pOnDeactivateModifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pOnDeactivateModifier")]
    POnDeactivateModifier(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsModifyOnceModifier<'de>, "@name",
    ("pOnActivateModifier" => POnActivateModifier(Cow<'de, str>)),
    ("pOnDeactivateModifier" => POnDeactivateModifier(Cow<'de, str>)),
}
