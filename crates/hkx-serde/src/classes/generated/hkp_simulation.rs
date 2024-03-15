//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimulation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSimulation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x97aba922`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimulation<'a> {
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
    /// -   name:`"determinismCheckFrameCounter"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "determinismCheckFrameCounter", default)]
    DeterminismCheckFrameCounter(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `struct hkpWorld*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "world", default)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"lastProcessingStep"`
    /// -   type: `enum LastProcessingStep`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastProcessingStep", default)]
    LastProcessingStep(Primitive<LastProcessingStep>),
    /// # C++ Class Fields Info
    /// -   name:`"currentTime"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentTime", default)]
    CurrentTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentPsiTime"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPsiTime", default)]
    CurrentPsiTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"physicsDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "physicsDeltaTime", default)]
    PhysicsDeltaTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"simulateUntilTime"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulateUntilTime", default)]
    SimulateUntilTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameMarkerPsiSnap", default)]
    FrameMarkerPsiSnap(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousStepResult"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousStepResult", default)]
    PreviousStepResult(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimulation<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("determinismCheckFrameCounter" => DeterminismCheckFrameCounter(Primitive<u32>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("lastProcessingStep" => LastProcessingStep(Primitive<LastProcessingStep>)),
    ("currentTime" => CurrentTime(Primitive<f32>)),
    ("currentPsiTime" => CurrentPsiTime(Primitive<f32>)),
    ("physicsDeltaTime" => PhysicsDeltaTime(Primitive<f32>)),
    ("simulateUntilTime" => SimulateUntilTime(Primitive<f32>)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(Primitive<f32>)),
    ("previousStepResult" => PreviousStepResult(Primitive<u32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FindContacts {
    #[serde(rename = "FIND_CONTACTS_DEFAULT")]
    FindContactsDefault = 0,
    #[serde(rename = "FIND_CONTACTS_EXTRA")]
    FindContactsExtra = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetCollisionInformation {
    #[serde(rename = "RESET_TOI")]
    ResetToi = 1,
    #[serde(rename = "RESET_TIM")]
    ResetTim = 2,
    #[serde(rename = "RESET_AABB")]
    ResetAabb = 4,
    #[serde(rename = "RESET_ALL")]
    ResetAll = 7,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LastProcessingStep {
    #[serde(rename = "INTEGRATE")]
    Integrate = 0,
    #[serde(rename = "COLLIDE")]
    Collide = 1,
}
