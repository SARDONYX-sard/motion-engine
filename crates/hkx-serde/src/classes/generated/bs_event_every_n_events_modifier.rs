//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSEventEveryNEventsModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSEventEveryNEventsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x6030970c`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsEventEveryNEventsModifier {
    /// # C++ Class Fields Info
    /// -   name:`"eventToCheckFor"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventToCheckFor")]
    EventToCheckFor(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"eventToSend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventToSend")]
    EventToSend(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"numberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfEventsBeforeSend")]
    NumberOfEventsBeforeSend(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"minimumNumberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 61
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minimumNumberOfEventsBeforeSend")]
    MinimumNumberOfEventsBeforeSend(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"randomizeNumberOfEvents"`
    /// -   type: `hkBool`
    /// - offset: 62
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "randomizeNumberOfEvents")]
    RandomizeNumberOfEvents(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"numberOfEventsSeen"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numberOfEventsSeen", skip_serializing)]
    NumberOfEventsSeen(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"calculatedNumberOfEventsBeforeSend"`
    /// -   type: `hkInt8`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "calculatedNumberOfEventsBeforeSend", skip_serializing)]
    CalculatedNumberOfEventsBeforeSend(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsEventEveryNEventsModifier, "@name",
    ("eventToCheckFor" => EventToCheckFor(HkbEventProperty)),
    ("eventToSend" => EventToSend(HkbEventProperty)),
    ("numberOfEventsBeforeSend" => NumberOfEventsBeforeSend(Primitive<i8>)),
    ("minimumNumberOfEventsBeforeSend" => MinimumNumberOfEventsBeforeSend(Primitive<i8>)),
    ("randomizeNumberOfEvents" => RandomizeNumberOfEvents(Primitive<bool>)),
    ("numberOfEventsSeen" => NumberOfEventsSeen(Primitive<i32>)),
    ("calculatedNumberOfEventsBeforeSend" => CalculatedNumberOfEventsBeforeSend(Primitive<i8>)),
}
