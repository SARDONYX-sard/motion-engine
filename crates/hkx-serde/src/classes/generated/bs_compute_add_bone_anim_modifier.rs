//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSComputeAddBoneAnimModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSComputeAddBoneAnimModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xa67f8c46`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsComputeAddBoneAnimModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"translationLSOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationLSOut")]
    TranslationLsOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationLSOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationLSOut")]
    RotationLsOut(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleLSOut"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleLSOut")]
    ScaleLsOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsComputeAddBoneAnimModifier<'de>, "@name",
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("translationLSOut" => TranslationLsOut(Vector4<f32>)),
    ("rotationLSOut" => RotationLsOut(Quaternion<f32>)),
    ("scaleLSOut" => ScaleLsOut(Vector4<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(Cow<'de, str>)),
}
