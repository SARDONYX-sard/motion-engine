//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventRangeData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventRangeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x6cb92c76`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventRangeData {
    /// # C++ Class Fields Info
    /// -   name:`"upperBound"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upperBound")]
    UpperBound(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventRangeMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventRangeMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventRangeData, "@name",
    ("upperBound" => UpperBound(Primitive<f32>)),
    ("event" => Event(HkbEventProperty)),
    ("eventMode" => EventMode(Primitive<EventRangeMode>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventRangeMode {
    #[serde(rename = "EVENT_MODE_SEND_ON_ENTER_RANGE")]
    EventModeSendOnEnterRange = 0,
    #[serde(rename = "EVENT_MODE_SEND_WHEN_IN_RANGE")]
    EventModeSendWhenInRange = 1,
}
