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
    /// # C++ Parent class(`hkbModifier`, parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable", default)]
    Enable(Primitive<bool>),
    /// # C++ Parent class(`hkbModifier`, parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padModifier", default, skip_serializing)]
    PadModifier([Primitive<bool>; 3]),

    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", default, skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", default, skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", default, skip_serializing)]
    PadNode([Primitive<bool>; 1]),

    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet", default)]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", default, skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", default, skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent1"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent1", default)]
    BEnableEvent1(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest1"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest1", default)]
    BVariableToTest1(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend1"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend1", default)]
    EventToSend1(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent2"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent2", default)]
    BEnableEvent2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest2"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest2", default)]
    BVariableToTest2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend2"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend2", default)]
    EventToSend2(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bEnableEvent3"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bEnableEvent3", default)]
    BEnableEvent3(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bVariableToTest3"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bVariableToTest3", default)]
    BVariableToTest3(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToSend3"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToSend3", default)]
    EventToSend3(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bSlot1ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot1ActivatedLastFrame", default, skip_serializing)]
    BSlot1ActivatedLastFrame(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bSlot2ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot2ActivatedLastFrame", default, skip_serializing)]
    BSlot2ActivatedLastFrame(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bSlot3ActivatedLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bSlot3ActivatedLastFrame", default, skip_serializing)]
    BSlot3ActivatedLastFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsEventOnFalseToTrueModifier, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier([Primitive<bool>; 3])),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<Unknown>)),
    ("padNode" => PadNode([Primitive<bool>; 1])),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<Primitive<()>>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
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
