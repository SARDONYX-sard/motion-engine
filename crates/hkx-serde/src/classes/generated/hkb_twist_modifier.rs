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
pub enum HkbTwistModifier {
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
    BoneChainIndices(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"parentBoneIndices"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parentBoneIndices", skip_serializing)]
    ParentBoneIndices(HkArrayRef<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTwistModifier, "@name",
    ("axisOfRotation" => AxisOfRotation(Vector4<f32>)),
    ("twistAngle" => TwistAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("setAngleMethod" => SetAngleMethod(Primitive<SetAngleMethod>)),
    ("rotationAxisCoordinates" => RotationAxisCoordinates(Primitive<RotationAxisCoordinates>)),
    ("isAdditive" => IsAdditive(Primitive<bool>)),
    ("boneChainIndices" => BoneChainIndices(HkArrayRef<()>)),
    ("parentBoneIndices" => ParentBoneIndices(HkArrayRef<()>)),
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
