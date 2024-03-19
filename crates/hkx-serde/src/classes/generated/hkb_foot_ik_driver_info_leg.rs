//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkDriverInfoLeg`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbFootIkDriverInfoLeg`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x224b18d1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkDriverInfoLeg {
    /// # C++ Class Fields Info
    /// -   name:`"prevAnkleRotLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "prevAnkleRotLS", skip_serializing)]
    PrevAnkleRotLs(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"kneeAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kneeAxisLS")]
    KneeAxisLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"footEndLS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footEndLS")]
    FootEndLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"footPlantedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footPlantedAnkleHeightMS")]
    FootPlantedAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footRaisedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footRaisedAnkleHeightMS")]
    FootRaisedAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAnkleHeightMS")]
    MaxAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAnkleHeightMS")]
    MinAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxKneeAngleDegrees")]
    MaxKneeAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minKneeAngleDegrees")]
    MinKneeAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAnkleAngleDegrees")]
    MaxAnkleAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"hipIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hipIndex")]
    HipIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"kneeIndex"`
    /// -   type: `hkInt16`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kneeIndex")]
    KneeIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"ankleIndex"`
    /// -   type: `hkInt16`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ankleIndex")]
    AnkleIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkDriverInfoLeg, "@name",
    ("prevAnkleRotLS" => PrevAnkleRotLs(Primitive<Quaternion<f32>>)),
    ("kneeAxisLS" => KneeAxisLs(Primitive<Vector4<f32>>)),
    ("footEndLS" => FootEndLs(Primitive<Vector4<f32>>)),
    ("footPlantedAnkleHeightMS" => FootPlantedAnkleHeightMs(Primitive<f32>)),
    ("footRaisedAnkleHeightMS" => FootRaisedAnkleHeightMs(Primitive<f32>)),
    ("maxAnkleHeightMS" => MaxAnkleHeightMs(Primitive<f32>)),
    ("minAnkleHeightMS" => MinAnkleHeightMs(Primitive<f32>)),
    ("maxKneeAngleDegrees" => MaxKneeAngleDegrees(Primitive<f32>)),
    ("minKneeAngleDegrees" => MinKneeAngleDegrees(Primitive<f32>)),
    ("maxAnkleAngleDegrees" => MaxAnkleAngleDegrees(Primitive<f32>)),
    ("hipIndex" => HipIndex(Primitive<i16>)),
    ("kneeIndex" => KneeIndex(Primitive<i16>)),
    ("ankleIndex" => AnkleIndex(Primitive<i16>)),
}
