//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbPoseMatchingGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbPoseMatchingGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbBlenderGenerator`/`0x22df7147`
/// - signature: `0x29e271b4`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoseMatchingGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModelRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelRotation")]
    WorldFromModelRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"blendSpeed"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendSpeed")]
    BlendSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minSpeedToSwitch"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSpeedToSwitch")]
    MinSpeedToSwitch(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minSwitchTimeNoError"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSwitchTimeNoError")]
    MinSwitchTimeNoError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minSwitchTimeFullError"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSwitchTimeFullError")]
    MinSwitchTimeFullError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startPlayingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startPlayingEventId")]
    StartPlayingEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"startMatchingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startMatchingEventId")]
    StartMatchingEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootBoneIndex")]
    RootBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "otherBoneIndex")]
    OtherBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anotherBoneIndex")]
    AnotherBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"pelvisIndex"`
    /// -   type: `hkInt16`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pelvisIndex")]
    PelvisIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum Mode`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(Mode),
    /// # C++ Class Fields Info
    /// -   name:`"currentMatch"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentMatch", skip_serializing)]
    CurrentMatch(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"bestMatch"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bestMatch", skip_serializing)]
    BestMatch(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceBetterMatch"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceBetterMatch", skip_serializing)]
    TimeSinceBetterMatch(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "error", skip_serializing)]
    Error(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"resetCurrentMatchLocalTime"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "resetCurrentMatchLocalTime", skip_serializing)]
    ResetCurrentMatchLocalTime(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingUtility"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "poseMatchingUtility", skip_serializing)]
    PoseMatchingUtility(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoseMatchingGenerator<'de>, "@name",
    ("worldFromModelRotation" => WorldFromModelRotation(Quaternion<f32>)),
    ("blendSpeed" => BlendSpeed(Primitive<f32>)),
    ("minSpeedToSwitch" => MinSpeedToSwitch(Primitive<f32>)),
    ("minSwitchTimeNoError" => MinSwitchTimeNoError(Primitive<f32>)),
    ("minSwitchTimeFullError" => MinSwitchTimeFullError(Primitive<f32>)),
    ("startPlayingEventId" => StartPlayingEventId(Primitive<i32>)),
    ("startMatchingEventId" => StartMatchingEventId(Primitive<i32>)),
    ("rootBoneIndex" => RootBoneIndex(Primitive<i16>)),
    ("otherBoneIndex" => OtherBoneIndex(Primitive<i16>)),
    ("anotherBoneIndex" => AnotherBoneIndex(Primitive<i16>)),
    ("pelvisIndex" => PelvisIndex(Primitive<i16>)),
    ("mode" => Mode(Mode)),
    ("currentMatch" => CurrentMatch(Primitive<i32>)),
    ("bestMatch" => BestMatch(Primitive<i32>)),
    ("timeSinceBetterMatch" => TimeSinceBetterMatch(Primitive<f32>)),
    ("error" => Error(Primitive<f32>)),
    ("resetCurrentMatchLocalTime" => ResetCurrentMatchLocalTime(Primitive<bool>)),
    ("poseMatchingUtility" => PoseMatchingUtility(Cow<'de, str>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "MODE_MATCH")]
    ModeMatch = 0,
    #[serde(rename = "MODE_PLAY")]
    ModePlay = 1,
}
