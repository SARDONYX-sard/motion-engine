//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSenseHandleModifierRange`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSenseHandleModifierRange`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0xfb56b692`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSenseHandleModifierRange {
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event", default)]
    Event(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"minDistance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDistance", default)]
    MinDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxDistance"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxDistance", default)]
    MaxDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ignoreHandle"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreHandle", default)]
    IgnoreHandle(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSenseHandleModifierRange, "@name",
    ("event" => Event(HkbEventProperty)),
    ("minDistance" => MinDistance(Primitive<f32>)),
    ("maxDistance" => MaxDistance(Primitive<f32>)),
    ("ignoreHandle" => IgnoreHandle(Primitive<bool>)),
}
