//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSTweenerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSTweenerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xd2d9a04`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsTweenerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"tweenPosition"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tweenPosition")]
    TweenPosition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"tweenRotation"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tweenRotation")]
    TweenRotation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"useTweenDuration"`
    /// -   type: `hkBool`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useTweenDuration")]
    UseTweenDuration(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"tweenDuration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tweenDuration")]
    TweenDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetRotation")]
    TargetRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "duration", skip_serializing)]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "startTransform", skip_serializing)]
    StartTransform(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsTweenerModifier, "@name",
    ("tweenPosition" => TweenPosition(Primitive<bool>)),
    ("tweenRotation" => TweenRotation(Primitive<bool>)),
    ("useTweenDuration" => UseTweenDuration(Primitive<bool>)),
    ("tweenDuration" => TweenDuration(Primitive<f32>)),
    ("targetPosition" => TargetPosition(Vector4<f32>)),
    ("targetRotation" => TargetRotation(Quaternion<f32>)),
    ("duration" => Duration(Primitive<f32>)),
    ("startTransform" => StartTransform(QsTransform<f32>)),
    ("time" => Time(Primitive<f32>)),
}
