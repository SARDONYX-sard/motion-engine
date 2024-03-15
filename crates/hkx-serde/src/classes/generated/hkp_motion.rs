//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMotion`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMotion`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 288
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x98aadb4f`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMotion<'a> {
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
    /// -   name:`"type"`
    /// -   type: `enum MotionType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<MotionType>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationNumInactiveFrames"`
    /// -   type: `hkUint16[2]`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFrames")]
    DeactivationNumInactiveFrames([Primitive<u16>; 2]),
    /// # C++ Class Fields Info
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionState")]
    MotionState(HkMotionState),
    /// # C++ Class Fields Info
    /// -   name:`"inertiaAndMassInv"`
    /// -   type: `hkVector4`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaAndMassInv")]
    InertiaAndMassInv(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocity")]
    LinearVelocity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angularVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularVelocity")]
    AngularVelocity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationRefPosition"`
    /// -   type: `hkVector4[2]`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationRefPosition")]
    DeactivationRefPosition([Vector4<f32>; 2]),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationRefOrientation"`
    /// -   type: `hkUint32[2]`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationRefOrientation")]
    DeactivationRefOrientation([Primitive<u32>; 2]),
    /// # C++ Class Fields Info
    /// -   name:`"savedMotion"`
    /// -   type: `struct hkpMaxSizeMotion*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "savedMotion")]
    SavedMotion(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"savedQualityTypeIndex"`
    /// -   type: `hkUint16`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "savedQualityTypeIndex")]
    SavedQualityTypeIndex(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"gravityFactor"`
    /// -   type: `hkHalf`
    /// - offset: 286
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravityFactor")]
    GravityFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMotion<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<MotionType>)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(Primitive<u8>)),
    ("deactivationNumInactiveFrames" => DeactivationNumInactiveFrames([Primitive<u16>; 2])),
    ("motionState" => MotionState(HkMotionState)),
    ("inertiaAndMassInv" => InertiaAndMassInv(Vector4<f32>)),
    ("linearVelocity" => LinearVelocity(Vector4<f32>)),
    ("angularVelocity" => AngularVelocity(Vector4<f32>)),
    ("deactivationRefPosition" => DeactivationRefPosition([Vector4<f32>; 2])),
    ("deactivationRefOrientation" => DeactivationRefOrientation([Primitive<u32>; 2])),
    ("savedMotion" => SavedMotion(Primitive<Cow<'de, str>>)),
    ("savedQualityTypeIndex" => SavedQualityTypeIndex(Primitive<u16>)),
    ("gravityFactor" => GravityFactor(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MotionType {
    #[serde(rename = "MOTION_INVALID")]
    MotionInvalid = 0,
    #[serde(rename = "MOTION_DYNAMIC")]
    MotionDynamic = 1,
    #[serde(rename = "MOTION_SPHERE_INERTIA")]
    MotionSphereInertia = 2,
    #[serde(rename = "MOTION_BOX_INERTIA")]
    MotionBoxInertia = 3,
    #[serde(rename = "MOTION_KEYFRAMED")]
    MotionKeyframed = 4,
    #[serde(rename = "MOTION_FIXED")]
    MotionFixed = 5,
    #[serde(rename = "MOTION_THIN_BOX_INERTIA")]
    MotionThinBoxInertia = 6,
    #[serde(rename = "MOTION_CHARACTER")]
    MotionCharacter = 7,
    #[serde(rename = "MOTION_MAX_ID")]
    MotionMaxId = 8,
}
