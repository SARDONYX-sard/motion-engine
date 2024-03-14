//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpTriggerVolume`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTriggerVolume`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa29a8d1a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriggerVolume<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"overlappingBodies"`
    /// -   type: `hkArray&lt;hkpRigidBody*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "overlappingBodies")]
    OverlappingBodies(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `hkArray&lt;struct hkpTriggerVolumeEventInfo&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventQueue")]
    EventQueue(HkArrayClass<HkpTriggerVolumeEventInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"triggerBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerBody")]
    TriggerBody(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"sequenceNumber"`
    /// -   type: `hkUint32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sequenceNumber")]
    SequenceNumber(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriggerVolume<'de>, "@name",
    ("overlappingBodies" => OverlappingBodies(HkArrayRef<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(HkArrayClass<HkpTriggerVolumeEventInfo>)),
    ("triggerBody" => TriggerBody(Cow<'de, str>)),
    ("sequenceNumber" => SequenceNumber(Primitive<u32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "ENTERED_EVENT")]
    EnteredEvent = 1,
    #[serde(rename = "LEFT_EVENT")]
    LeftEvent = 2,
    #[serde(rename = "ENTERED_AND_LEFT_EVENT")]
    EnteredAndLeftEvent = 3,
    #[serde(rename = "TRIGGER_BODY_LEFT_EVENT")]
    TriggerBodyLeftEvent = 6,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "ADDED_OP")]
    AddedOp = 0,
    #[serde(rename = "REMOVED_OP")]
    RemovedOp = 1,
    #[serde(rename = "CONTACT_OP")]
    ContactOp = 2,
    #[serde(rename = "TOI_OP")]
    ToiOp = 3,
}
