//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSLimbIKModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSLimbIKModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x8ea971e5`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsLimbIkModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentAngle"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentAngle", skip_serializing)]
    CurrentAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"boneRadius"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneRadius")]
    BoneRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"castOffset"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "castOffset")]
    CastOffset(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsLimbIkModifier<'de>, "@name",
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("currentAngle" => CurrentAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("gain" => Gain(Primitive<f32>)),
    ("boneRadius" => BoneRadius(Primitive<f32>)),
    ("castOffset" => CastOffset(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(Primitive<Cow<'de, str>>)),
}
