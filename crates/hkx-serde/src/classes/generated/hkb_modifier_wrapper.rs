//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbModifierWrapper`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbModifierWrapper`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x3697e044`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbModifierWrapper<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"modifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modifier")]
    Modifier(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbModifierWrapper<'de>, "@name",
    ("modifier" => Modifier(Cow<'de, str>)),
}
