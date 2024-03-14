//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbTimerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbTimerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x338b4879`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTimerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"alarmTimeSeconds"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alarmTimeSeconds")]
    AlarmTimeSeconds(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"alarmEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alarmEvent")]
    AlarmEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"secondsElapsed"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "secondsElapsed", skip_serializing)]
    SecondsElapsed(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTimerModifier, "@name",
    ("alarmTimeSeconds" => AlarmTimeSeconds(Primitive<f32>)),
    ("alarmEvent" => AlarmEvent(HkbEventProperty)),
    ("secondsElapsed" => SecondsElapsed(Primitive<f32>)),
}
