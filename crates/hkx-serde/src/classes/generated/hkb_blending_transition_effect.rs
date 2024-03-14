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
    FromGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPoseAtBeginningOfTransition", skip_serializing)]
    CharacterPoseAtBeginningOfTransition(HkArrayRef<()>),
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
    ("duration" => Duration(Primitive<f32>)),
    ("toGeneratorStartTimeFraction" => ToGeneratorStartTimeFraction(Primitive<f32>)),
    ("flags" => Flags(Primitive<FlagBits>)),
    ("endMode" => EndMode(Primitive<EndMode>)),
    ("blendCurve" => BlendCurve(Primitive<BlendCurve>)),
    ("fromGenerator" => FromGenerator(Cow<'de, str>)),
    ("toGenerator" => ToGenerator(Cow<'de, str>)),
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(HkArrayRef<()>)),
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
