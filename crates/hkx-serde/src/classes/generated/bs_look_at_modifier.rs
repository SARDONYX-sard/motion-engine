//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSLookAtModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSLookAtModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xd756fc25`
/// -   version: 4
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsLookAtModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"lookAtTarget"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtTarget")]
    LookAtTarget(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `hkArray&lt;struct BSLookAtModifierBoneData&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(HkArrayClass<BsLookAtModifierBoneData>),
    /// # C++ Class Fields Info
    /// -   name:`"eyeBones"`
    /// -   type: `hkArray&lt;struct BSLookAtModifierBoneData&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eyeBones")]
    EyeBones(HkArrayClass<BsLookAtModifierBoneData>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleThresholdDegrees"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleThresholdDegrees")]
    LimitAngleThresholdDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"continueLookOutsideOfLimit"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "continueLookOutsideOfLimit")]
    ContinueLookOutsideOfLimit(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"useBoneGains"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useBoneGains")]
    UseBoneGains(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"targetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetLocation")]
    TargetLocation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetOutsideLimits"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetOutsideLimits")]
    TargetOutsideLimits(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"targetOutOfLimitEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetOutOfLimitEvent")]
    TargetOutOfLimitEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCamera"`
    /// -   type: `hkBool`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCamera")]
    LookAtCamera(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCameraX"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCameraX")]
    LookAtCameraX(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCameraY"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCameraY")]
    LookAtCameraY(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"lookAtCameraZ"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtCameraZ")]
    LookAtCameraZ(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ballBonesValid"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ballBonesValid", skip_serializing)]
    BallBonesValid(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsLookAtModifier<'de>, "@name",
    ("lookAtTarget" => LookAtTarget(Primitive<bool>)),
    ("bones" => Bones(HkArrayClass<BsLookAtModifierBoneData>)),
    ("eyeBones" => EyeBones(HkArrayClass<BsLookAtModifierBoneData>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("limitAngleThresholdDegrees" => LimitAngleThresholdDegrees(Primitive<f32>)),
    ("continueLookOutsideOfLimit" => ContinueLookOutsideOfLimit(Primitive<bool>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("useBoneGains" => UseBoneGains(Primitive<bool>)),
    ("targetLocation" => TargetLocation(Vector4<f32>)),
    ("targetOutsideLimits" => TargetOutsideLimits(Primitive<bool>)),
    ("targetOutOfLimitEvent" => TargetOutOfLimitEvent(HkbEventProperty)),
    ("lookAtCamera" => LookAtCamera(Primitive<bool>)),
    ("lookAtCameraX" => LookAtCameraX(Primitive<f32>)),
    ("lookAtCameraY" => LookAtCameraY(Primitive<f32>)),
    ("lookAtCameraZ" => LookAtCameraZ(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("ballBonesValid" => BallBonesValid(Primitive<bool>)),
    ("pSkeletonMemory" => PSkeletonMemory(Cow<'de, str>)),
}
