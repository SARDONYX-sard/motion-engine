//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkModifierHand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbHandIkModifierHand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x14dfe1dd`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkModifierHand<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"elbowAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowAxisLS")]
    ElbowAxisLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"backHandNormalLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "backHandNormalLS")]
    BackHandNormalLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"handOffsetLS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handOffsetLS")]
    HandOffsetLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"handOrienationOffsetLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handOrienationOffsetLS")]
    HandOrienationOffsetLs(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"maxElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxElbowAngleDegrees")]
    MaxElbowAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minElbowAngleDegrees")]
    MinElbowAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"shoulderIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shoulderIndex")]
    ShoulderIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"shoulderSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shoulderSiblingIndex")]
    ShoulderSiblingIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"elbowIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowIndex")]
    ElbowIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"elbowSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elbowSiblingIndex")]
    ElbowSiblingIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"wristIndex"`
    /// -   type: `hkInt16`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wristIndex")]
    WristIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"enforceEndPosition"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforceEndPosition")]
    EnforceEndPosition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"enforceEndRotation"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforceEndRotation")]
    EnforceEndRotation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkModifierHand<'de>, "@name",
    ("elbowAxisLS" => ElbowAxisLs(Primitive<Vector4<f32>>)),
    ("backHandNormalLS" => BackHandNormalLs(Primitive<Vector4<f32>>)),
    ("handOffsetLS" => HandOffsetLs(Primitive<Vector4<f32>>)),
    ("handOrienationOffsetLS" => HandOrienationOffsetLs(Primitive<Quaternion<f32>>)),
    ("maxElbowAngleDegrees" => MaxElbowAngleDegrees(Primitive<f32>)),
    ("minElbowAngleDegrees" => MinElbowAngleDegrees(Primitive<f32>)),
    ("shoulderIndex" => ShoulderIndex(Primitive<i16>)),
    ("shoulderSiblingIndex" => ShoulderSiblingIndex(Primitive<i16>)),
    ("elbowIndex" => ElbowIndex(Primitive<i16>)),
    ("elbowSiblingIndex" => ElbowSiblingIndex(Primitive<i16>)),
    ("wristIndex" => WristIndex(Primitive<i16>)),
    ("enforceEndPosition" => EnforceEndPosition(Primitive<bool>)),
    ("enforceEndRotation" => EnforceEndRotation(Primitive<bool>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkbHandIkModifierHand<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
