//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSDirectAtModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSDirectAtModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x19a005c0`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsDirectAtModifier<'a> {
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
    /// -   name:`"directAtTarget"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtTarget", default)]
    DirectAtTarget(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"sourceBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sourceBoneIndex", default)]
    SourceBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneIndex", default)]
    StartBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneIndex", default)]
    EndBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"limitHeadingDegrees"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitHeadingDegrees", default)]
    LimitHeadingDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitPitchDegrees"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitPitchDegrees", default)]
    LimitPitchDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"offsetHeadingDegrees"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetHeadingDegrees", default)]
    OffsetHeadingDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"offsetPitchDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetPitchDegrees", default)]
    OffsetPitchDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain", default)]
    OnGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain", default)]
    OffGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetLocation", default)]
    TargetLocation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"userInfo"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userInfo", default)]
    UserInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"directAtCamera"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCamera", default)]
    DirectAtCamera(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"directAtCameraX"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCameraX", default)]
    DirectAtCameraX(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"directAtCameraY"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCameraY", default)]
    DirectAtCameraY(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"directAtCameraZ"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directAtCameraZ", default)]
    DirectAtCameraZ(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "active", default)]
    Active(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"currentHeadingOffset"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentHeadingOffset", default)]
    CurrentHeadingOffset(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentPitchOffset"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPitchOffset", default)]
    CurrentPitchOffset(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", default, skip_serializing)]
    TimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", default, skip_serializing)]
    PSkeletonMemory(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"hasTarget"`
    /// -   type: `hkBool`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "hasTarget", default, skip_serializing)]
    HasTarget(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"directAtTargetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "directAtTargetLocation", default, skip_serializing)]
    DirectAtTargetLocation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"boneChainIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boneChainIndices", default, skip_serializing)]
    BoneChainIndices(HkArrayRef<Primitive<()>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsDirectAtModifier<'de>, "@name",
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
    ("directAtTarget" => DirectAtTarget(Primitive<bool>)),
    ("sourceBoneIndex" => SourceBoneIndex(Primitive<i16>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("limitHeadingDegrees" => LimitHeadingDegrees(Primitive<f32>)),
    ("limitPitchDegrees" => LimitPitchDegrees(Primitive<f32>)),
    ("offsetHeadingDegrees" => OffsetHeadingDegrees(Primitive<f32>)),
    ("offsetPitchDegrees" => OffsetPitchDegrees(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("targetLocation" => TargetLocation(Vector4<f32>)),
    ("userInfo" => UserInfo(Primitive<u32>)),
    ("directAtCamera" => DirectAtCamera(Primitive<bool>)),
    ("directAtCameraX" => DirectAtCameraX(Primitive<f32>)),
    ("directAtCameraY" => DirectAtCameraY(Primitive<f32>)),
    ("directAtCameraZ" => DirectAtCameraZ(Primitive<f32>)),
    ("active" => Active(Primitive<bool>)),
    ("currentHeadingOffset" => CurrentHeadingOffset(Primitive<f32>)),
    ("currentPitchOffset" => CurrentPitchOffset(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(Primitive<Cow<'de, str>>)),
    ("hasTarget" => HasTarget(Primitive<bool>)),
    ("directAtTargetLocation" => DirectAtTargetLocation(Vector4<f32>)),
    ("boneChainIndices" => BoneChainIndices(HkArrayRef<Primitive<()>>)),
}
