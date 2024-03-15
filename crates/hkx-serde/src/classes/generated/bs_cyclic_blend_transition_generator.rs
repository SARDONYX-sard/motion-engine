//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSCyclicBlendTransitionGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSCyclicBlendTransitionGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x5119eb06`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsCyclicBlendTransitionGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pBlenderGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pBlenderGenerator")]
    PBlenderGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToFreezeBlendValue"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToFreezeBlendValue")]
    EventToFreezeBlendValue(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"EventToCrossBlend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToCrossBlend")]
    EventToCrossBlend(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"fBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fBlendParameter")]
    FBlendParameter(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fTransitionDuration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fTransitionDuration")]
    FTransitionDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"eBlendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eBlendCurve")]
    EBlendCurve(Primitive<BlendCurve>),
    /// # C++ Class Fields Info
    /// -   name:`"pTransitionBlenderGenerator"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | ALIGN16 | SERIALIZE_IGNORED`
    #[serde(rename = "pTransitionBlenderGenerator", skip_serializing)]
    PTransitionBlenderGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"pTransitionEffect"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | ALIGN16 | SERIALIZE_IGNORED`
    #[serde(rename = "pTransitionEffect", skip_serializing)]
    PTransitionEffect(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"currentMode"`
    /// -   type: `enum unknown`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentMode", skip_serializing)]
    CurrentMode(Primitive<Unknown>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsCyclicBlendTransitionGenerator<'de>, "@name",
    ("pBlenderGenerator" => PBlenderGenerator(Primitive<Cow<'de, str>>)),
    ("EventToFreezeBlendValue" => EventToFreezeBlendValue(HkbEventProperty)),
    ("EventToCrossBlend" => EventToCrossBlend(HkbEventProperty)),
    ("fBlendParameter" => FBlendParameter(Primitive<f32>)),
    ("fTransitionDuration" => FTransitionDuration(Primitive<f32>)),
    ("eBlendCurve" => EBlendCurve(Primitive<BlendCurve>)),
    ("pTransitionBlenderGenerator" => PTransitionBlenderGenerator(Primitive<Cow<'de, str>>)),
    ("pTransitionEffect" => PTransitionEffect(Primitive<Cow<'de, str>>)),
    ("currentMode" => CurrentMode(Primitive<Unknown>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CurrentBlendMode {
    #[serde(rename = "MODE_INACTIVE")]
    ModeInactive = -1,
    #[serde(rename = "MODE_DEFAULT")]
    ModeDefault = 0,
    #[serde(rename = "MODE_FROZEN")]
    ModeFrozen = 1,
    #[serde(rename = "MODE_BLENDING")]
    ModeBlending = 2,
    #[serde(rename = "MODE_WAITINGFORBLENDING")]
    ModeWaitingforblending = 3,
}
