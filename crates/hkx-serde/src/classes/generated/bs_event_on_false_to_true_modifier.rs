//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSEventOnFalseToTrueModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSEventOnFalseToTrueModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 84
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x81d0777a`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsEventOnFalseToTrueModifier {
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent1"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent1")]
    BEnableEvent1(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest1"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest1")]
    BVariableToTest1(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend1"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend1")]
    EventToSend1(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent2"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent2")]
    BEnableEvent2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest2"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest2")]
    BVariableToTest2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend2"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend2")]
    EventToSend2(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent3"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent3")]
    BEnableEvent3(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest3"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest3")]
    BVariableToTest3(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend3"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend3")]
    EventToSend3(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bSlot1ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot1ActivatedLastFrame", skip_serializing)]
    BSlot1ActivatedLastFrame(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bSlot2ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot2ActivatedLastFrame", skip_serializing)]
    BSlot2ActivatedLastFrame(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bSlot3ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot3ActivatedLastFrame", skip_serializing)]
    BSlot3ActivatedLastFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsEventOnFalseToTrueModifier, "@name",
    ("bEnableEvent1" => BEnableEvent1(Primitive<bool>)),
    ("bVariableToTest1" => BVariableToTest1(Primitive<bool>)),
    ("EventToSend1" => EventToSend1(HkbEventProperty)),
    ("bEnableEvent2" => BEnableEvent2(Primitive<bool>)),
    ("bVariableToTest2" => BVariableToTest2(Primitive<bool>)),
    ("EventToSend2" => EventToSend2(HkbEventProperty)),
    ("bEnableEvent3" => BEnableEvent3(Primitive<bool>)),
    ("bVariableToTest3" => BVariableToTest3(Primitive<bool>)),
    ("EventToSend3" => EventToSend3(HkbEventProperty)),
    ("bSlot1ActivatedLastFrame" => BSlot1ActivatedLastFrame(Primitive<bool>)),
    ("bSlot2ActivatedLastFrame" => BSlot2ActivatedLastFrame(Primitive<bool>)),
    ("bSlot3ActivatedLastFrame" => BSlot3ActivatedLastFrame(Primitive<bool>)),
}
