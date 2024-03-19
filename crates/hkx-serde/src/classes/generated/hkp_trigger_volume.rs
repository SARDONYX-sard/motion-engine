//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriggerVolume`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriggerVolume<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"overlappingBodies"`
    /// -   type: `hkArray<hkpRigidBody*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "overlappingBodies")]
    OverlappingBodies(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `hkArray<struct hkpTriggerVolumeEventInfo>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventQueue")]
    EventQueue(HkArrayClass<HkpTriggerVolumeEventInfo<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"triggerBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerBody")]
    TriggerBody(Primitive<Cow<'a, str>>),
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
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("overlappingBodies" => OverlappingBodies(HkArrayRef<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(HkArrayClass<HkpTriggerVolumeEventInfo<'de>>)),
    ("triggerBody" => TriggerBody(Primitive<Cow<'de, str>>)),
    ("sequenceNumber" => SequenceNumber(Primitive<u32>)),
}

#[allow(clippy::enum_variant_names)]
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

#[allow(clippy::enum_variant_names)]
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
