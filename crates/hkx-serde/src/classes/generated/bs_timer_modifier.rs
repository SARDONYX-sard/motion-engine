//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSTimerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSTimerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x531f3292`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsTimerModifier {
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
    /// -   name:`"resetAlarm"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resetAlarm")]
    ResetAlarm(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"secondsElapsed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "secondsElapsed", skip_serializing)]
    SecondsElapsed(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsTimerModifier, "@name",
    ("alarmTimeSeconds" => AlarmTimeSeconds(Primitive<f32>)),
    ("alarmEvent" => AlarmEvent(HkbEventProperty)),
    ("resetAlarm" => ResetAlarm(Primitive<bool>)),
    ("secondsElapsed" => SecondsElapsed(Primitive<f32>)),
}
