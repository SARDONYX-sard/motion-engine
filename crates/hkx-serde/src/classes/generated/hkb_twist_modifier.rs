//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTwistModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbTwistModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xb6b76b32`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTwistModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<bool, 3>),

    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<bool, 1>),

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

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

    /// # C++ Class Fields Info
    /// -   name:`"axisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axisOfRotation")]
    AxisOfRotation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"twistAngle"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistAngle")]
    TwistAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"setAngleMethod"`
    /// -   type: `enum SetAngleMethod`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setAngleMethod")]
    SetAngleMethod(Primitive<SetAngleMethod>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationAxisCoordinates"`
    /// -   type: `enum RotationAxisCoordinates`
    /// - offset: 73
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationAxisCoordinates")]
    RotationAxisCoordinates(Primitive<RotationAxisCoordinates>),
    /// # C++ Class Fields Info
    /// -   name:`"isAdditive"`
    /// -   type: `hkBool`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isAdditive")]
    IsAdditive(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"boneChainIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boneChainIndices", skip_serializing)]
    BoneChainIndices(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"parentBoneIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parentBoneIndices", skip_serializing)]
    ParentBoneIndices(HkArrayRef<Primitive<()>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTwistModifier<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<bool, 3>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<Unknown>)),
    ("padNode" => PadNode(CStyleArray<bool, 1>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<Primitive<()>>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("axisOfRotation" => AxisOfRotation(Vector4<f32>)),
    ("twistAngle" => TwistAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("setAngleMethod" => SetAngleMethod(Primitive<SetAngleMethod>)),
    ("rotationAxisCoordinates" => RotationAxisCoordinates(Primitive<RotationAxisCoordinates>)),
    ("isAdditive" => IsAdditive(Primitive<bool>)),
    ("boneChainIndices" => BoneChainIndices(HkArrayRef<Primitive<()>>)),
    ("parentBoneIndices" => ParentBoneIndices(HkArrayRef<Primitive<()>>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAngleMethod {
    #[serde(rename = "LINEAR")]
    Linear = 0,
    #[serde(rename = "RAMPED")]
    Ramped = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RotationAxisCoordinates {
    #[serde(rename = "ROTATION_AXIS_IN_MODEL_COORDINATES")]
    RotationAxisInModelCoordinates = 0,
    #[serde(rename = "ROTATION_AXIS_IN_LOCAL_COORDINATES")]
    RotationAxisInLocalCoordinates = 1,
}
