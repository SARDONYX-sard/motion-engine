//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpThinBoxMotion`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpThinBoxMotion`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 288
/// -    vtable: true
/// -    parent: `hkpBoxMotion`/`0xbafa2bb7`
/// - signature: `0x64abf85c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpThinBoxMotion<'a> {
    // C++ Parent class(`hkpBoxMotion` => parent: `hkpMotion`) has no fields

    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum MotionType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<MotionType>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationNumInactiveFrames"`
    /// -   type: `hkUint16[2]`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFrames")]
    DeactivationNumInactiveFrames(CStyleArray<u16, 2>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionState")]
    MotionState(HkMotionState),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"inertiaAndMassInv"`
    /// -   type: `hkVector4`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaAndMassInv")]
    InertiaAndMassInv(Vector4<f32>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocity")]
    LinearVelocity(Vector4<f32>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"angularVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularVelocity")]
    AngularVelocity(Vector4<f32>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationRefPosition"`
    /// -   type: `hkVector4[2]`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationRefPosition")]
    DeactivationRefPosition(CStyleArrayVector<Vector4<f32>, 2>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"deactivationRefOrientation"`
    /// -   type: `hkUint32[2]`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationRefOrientation")]
    DeactivationRefOrientation(CStyleArray<u32, 2>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"savedMotion"`
    /// -   type: `struct hkpMaxSizeMotion*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "savedMotion")]
    SavedMotion(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"savedQualityTypeIndex"`
    /// -   type: `hkUint16`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "savedQualityTypeIndex")]
    SavedQualityTypeIndex(Primitive<u16>),
    /// # C++ Parent class(`hkpMotion` => parent: `hkReferencedObject`) field Info
    /// -   name:`"gravityFactor"`
    /// -   type: `hkHalf`
    /// - offset: 286
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravityFactor")]
    GravityFactor(Primitive<f32>),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpThinBoxMotion<'de>, "@name",
    ("type" => Type(Primitive<MotionType>)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(Primitive<u8>)),
    ("deactivationNumInactiveFrames" => DeactivationNumInactiveFrames(CStyleArray<u16, 2>)),
    ("motionState" => MotionState(HkMotionState)),
    ("inertiaAndMassInv" => InertiaAndMassInv(Vector4<f32>)),
    ("linearVelocity" => LinearVelocity(Vector4<f32>)),
    ("angularVelocity" => AngularVelocity(Vector4<f32>)),
    ("deactivationRefPosition" => DeactivationRefPosition(CStyleArrayVector<Vector4<f32>, 2>)),
    ("deactivationRefOrientation" => DeactivationRefOrientation(CStyleArray<u32, 2>)),
    ("savedMotion" => SavedMotion(Primitive<Cow<'de, str>>)),
    ("savedQualityTypeIndex" => SavedQualityTypeIndex(Primitive<u16>)),
    ("gravityFactor" => GravityFactor(Primitive<f32>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}
