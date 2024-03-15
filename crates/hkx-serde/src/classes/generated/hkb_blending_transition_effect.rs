//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendingTransitionEffect`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBlendingTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkbTransitionEffect`/`0x945da157`
/// - signature: `0xfd8584fe`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendingTransitionEffect<'a> {
    /// # C++ Parent class(`hkbTransitionEffect`, parent: `hkbGenerator`) field Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<SelfTransitionMode>),
    /// # C++ Parent class(`hkbTransitionEffect`, parent: `hkbGenerator`) field Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventMode>),
    /// # C++ Parent class(`hkbTransitionEffect`, parent: `hkbGenerator`) field Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Primitive<Unknown>),

    // `hkbGenerator`(Parent class) has no fields

    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode([Primitive<bool>; 1]),

    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorStartTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toGeneratorStartTimeFraction")]
    ToGeneratorStartTimeFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagBits`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<FlagBits>),
    /// # C++ Class Fields Info
    /// -   name:`"endMode"`
    /// -   type: `enum EndMode`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endMode")]
    EndMode(Primitive<EndMode>),
    /// # C++ Class Fields Info
    /// -   name:`"blendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 55
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendCurve")]
    BlendCurve(Primitive<BlendCurve>),
    /// # C++ Class Fields Info
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPoseAtBeginningOfTransition", skip_serializing)]
    CharacterPoseAtBeginningOfTransition(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeRemaining", skip_serializing)]
    TimeRemaining(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "applySelfTransition", skip_serializing)]
    ApplySelfTransition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "initializeCharacterPose", skip_serializing)]
    InitializeCharacterPose(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffect<'de>, "@name",
    ("selfTransitionMode" => SelfTransitionMode(Primitive<SelfTransitionMode>)),
    ("eventMode" => EventMode(Primitive<EventMode>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<Unknown>)),
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
    ("duration" => Duration(Primitive<f32>)),
    ("toGeneratorStartTimeFraction" => ToGeneratorStartTimeFraction(Primitive<f32>)),
    ("flags" => Flags(Primitive<FlagBits>)),
    ("endMode" => EndMode(Primitive<EndMode>)),
    ("blendCurve" => BlendCurve(Primitive<BlendCurve>)),
    ("fromGenerator" => FromGenerator(Primitive<Cow<'de, str>>)),
    ("toGenerator" => ToGenerator(Primitive<Cow<'de, str>>)),
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(HkArrayRef<Primitive<()>>)),
    ("timeRemaining" => TimeRemaining(Primitive<f32>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("applySelfTransition" => ApplySelfTransition(Primitive<bool>)),
    ("initializeCharacterPose" => InitializeCharacterPose(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlagBits {
    #[serde(rename = "FLAG_NONE")]
    FlagNone = 0,
    #[serde(rename = "FLAG_IGNORE_FROM_WORLD_FROM_MODEL")]
    FlagIgnoreFromWorldFromModel = 1,
    #[serde(rename = "FLAG_SYNC")]
    FlagSync = 2,
    #[serde(rename = "FLAG_IGNORE_TO_WORLD_FROM_MODEL")]
    FlagIgnoreToWorldFromModel = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndMode {
    #[serde(rename = "END_MODE_NONE")]
    EndModeNone = 0,
    #[serde(rename = "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR")]
    EndModeTransitionUntilEndOfFromGenerator = 1,
    #[serde(rename = "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR")]
    EndModeCapDurationAtEndOfFromGenerator = 2,
}
