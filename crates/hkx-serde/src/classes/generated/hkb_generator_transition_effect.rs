//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGeneratorTransitionEffect`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbGeneratorTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 92
/// -    vtable: true
/// -    parent: `hkbTransitionEffect`/`0x945da157`
/// - signature: `0x5f771b12`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorTransitionEffect<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transitionGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionGenerator")]
    TransitionGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"blendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendInDuration")]
    BlendInDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"blendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendOutDuration")]
    BlendOutDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"syncToGeneratorStartTime"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncToGeneratorStartTime")]
    SyncToGeneratorStartTime(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "duration", skip_serializing)]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "effectiveBlendInDuration", skip_serializing)]
    EffectiveBlendInDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "effectiveBlendOutDuration", skip_serializing)]
    EffectiveBlendOutDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorState"`
    /// -   type: `enum unknown`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "toGeneratorState", skip_serializing)]
    ToGeneratorState(Primitive<Unknown>),
    /// # C++ Class Fields Info
    /// -   name:`"echoTransitionGenerator"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoTransitionGenerator", skip_serializing)]
    EchoTransitionGenerator(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"echoToGenerator"`
    /// -   type: `hkBool`
    /// - offset: 86
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoToGenerator", skip_serializing)]
    EchoToGenerator(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"justActivated"`
    /// -   type: `hkBool`
    /// - offset: 87
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "justActivated", skip_serializing)]
    JustActivated(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "updateActiveNodes", skip_serializing)]
    UpdateActiveNodes(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"stage"`
    /// -   type: `enum unknown`
    /// - offset: 89
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stage", skip_serializing)]
    Stage(Primitive<Unknown>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorTransitionEffect<'de>, "@name",
    ("transitionGenerator" => TransitionGenerator(Primitive<Cow<'de, str>>)),
    ("blendInDuration" => BlendInDuration(Primitive<f32>)),
    ("blendOutDuration" => BlendOutDuration(Primitive<f32>)),
    ("syncToGeneratorStartTime" => SyncToGeneratorStartTime(Primitive<bool>)),
    ("fromGenerator" => FromGenerator(Primitive<Cow<'de, str>>)),
    ("toGenerator" => ToGenerator(Primitive<Cow<'de, str>>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
    ("effectiveBlendInDuration" => EffectiveBlendInDuration(Primitive<f32>)),
    ("effectiveBlendOutDuration" => EffectiveBlendOutDuration(Primitive<f32>)),
    ("toGeneratorState" => ToGeneratorState(Primitive<Unknown>)),
    ("echoTransitionGenerator" => EchoTransitionGenerator(Primitive<bool>)),
    ("echoToGenerator" => EchoToGenerator(Primitive<bool>)),
    ("justActivated" => JustActivated(Primitive<bool>)),
    ("updateActiveNodes" => UpdateActiveNodes(Primitive<bool>)),
    ("stage" => Stage(Primitive<Unknown>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToGeneratorState {
    #[serde(rename = "STATE_INACTIVE")]
    StateInactive = 0,
    #[serde(rename = "STATE_READY_FOR_SET_LOCAL_TIME")]
    StateReadyForSetLocalTime = 1,
    #[serde(rename = "STATE_READY_FOR_APPLY_SELF_TRANSITION_MODE")]
    StateReadyForApplySelfTransitionMode = 2,
    #[serde(rename = "STATE_ACTIVE")]
    StateActive = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stage {
    #[serde(rename = "STAGE_BLENDING_IN")]
    StageBlendingIn = 0,
    #[serde(rename = "STAGE_PLAYING_TRANSITION_GENERATOR")]
    StagePlayingTransitionGenerator = 1,
    #[serde(rename = "STAGE_BLENDING_OUT")]
    StageBlendingOut = 2,
}
