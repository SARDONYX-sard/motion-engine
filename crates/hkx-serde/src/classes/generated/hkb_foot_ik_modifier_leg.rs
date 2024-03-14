//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkModifierLeg`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbFootIkModifierLeg`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x9f3e3a04`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifierLeg {
    /// # C++ Class Fields Info
    /// -   name:`"originalAnkleTransformMS"`
    /// -   type: `hkQsTransform`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalAnkleTransformMS")]
    OriginalAnkleTransformMs(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"prevAnkleRotLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "prevAnkleRotLS", skip_serializing)]
    PrevAnkleRotLs(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"kneeAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kneeAxisLS")]
    KneeAxisLs(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footEndLS"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footEndLS")]
    FootEndLs(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ungroundedEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ungroundedEvent")]
    UngroundedEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"footPlantedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footPlantedAnkleHeightMS")]
    FootPlantedAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footRaisedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footRaisedAnkleHeightMS")]
    FootRaisedAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAnkleHeightMS")]
    MaxAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAnkleHeightMS")]
    MinAnkleHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxKneeAngleDegrees")]
    MaxKneeAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minKneeAngleDegrees")]
    MinKneeAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalError"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalError")]
    VerticalError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAnkleAngleDegrees")]
    MaxAnkleAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"hipIndex"`
    /// -   type: `hkInt16`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hipIndex")]
    HipIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"kneeIndex"`
    /// -   type: `hkInt16`
    /// - offset: 138
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kneeIndex")]
    KneeIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"ankleIndex"`
    /// -   type: `hkInt16`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ankleIndex")]
    AnkleIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"hitSomething"`
    /// -   type: `hkBool`
    /// - offset: 142
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hitSomething")]
    HitSomething(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isPlantedMS"`
    /// -   type: `hkBool`
    /// - offset: 143
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isPlantedMS")]
    IsPlantedMs(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isOriginalAnkleTransformMSSet"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isOriginalAnkleTransformMSSet")]
    IsOriginalAnkleTransformMsSet(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierLeg, "@name",
    ("originalAnkleTransformMS" => OriginalAnkleTransformMs(QsTransform<f32>)),
    ("prevAnkleRotLS" => PrevAnkleRotLs(Quaternion<f32>)),
    ("kneeAxisLS" => KneeAxisLs(Vector4<f32>)),
    ("footEndLS" => FootEndLs(Vector4<f32>)),
    ("ungroundedEvent" => UngroundedEvent(HkbEventProperty)),
    ("footPlantedAnkleHeightMS" => FootPlantedAnkleHeightMs(Primitive<f32>)),
    ("footRaisedAnkleHeightMS" => FootRaisedAnkleHeightMs(Primitive<f32>)),
    ("maxAnkleHeightMS" => MaxAnkleHeightMs(Primitive<f32>)),
    ("minAnkleHeightMS" => MinAnkleHeightMs(Primitive<f32>)),
    ("maxKneeAngleDegrees" => MaxKneeAngleDegrees(Primitive<f32>)),
    ("minKneeAngleDegrees" => MinKneeAngleDegrees(Primitive<f32>)),
    ("verticalError" => VerticalError(Primitive<f32>)),
    ("maxAnkleAngleDegrees" => MaxAnkleAngleDegrees(Primitive<f32>)),
    ("hipIndex" => HipIndex(Primitive<i16>)),
    ("kneeIndex" => KneeIndex(Primitive<i16>)),
    ("ankleIndex" => AnkleIndex(Primitive<i16>)),
    ("hitSomething" => HitSomething(Primitive<bool>)),
    ("isPlantedMS" => IsPlantedMs(Primitive<bool>)),
    ("isOriginalAnkleTransformMSSet" => IsOriginalAnkleTransformMsSet(Primitive<bool>)),
}
