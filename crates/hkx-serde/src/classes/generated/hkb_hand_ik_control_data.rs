//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbHandIkControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xd72b8d17`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkControlData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition", default)]
    TargetPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetRotation", default)]
    TargetRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetNormal"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetNormal", default)]
    TargetNormal(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetHandle", default)]
    TargetHandle(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"transformOnFraction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformOnFraction", default)]
    TransformOnFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"normalOnFraction"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalOnFraction", default)]
    NormalOnFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fadeInDuration"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeInDuration", default)]
    FadeInDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fadeOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeOutDuration", default)]
    FadeOutDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep", default)]
    ExtrapolationTimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeSpeed", default)]
    HandleChangeSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeMode", default)]
    HandleChangeMode(Primitive<HandleChangeMode>),
    /// # C++ Class Fields Info
    /// -   name:`"fixUp"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixUp", default)]
    FixUp(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkControlData<'de>, "@name",
    ("targetPosition" => TargetPosition(Vector4<f32>)),
    ("targetRotation" => TargetRotation(Quaternion<f32>)),
    ("targetNormal" => TargetNormal(Vector4<f32>)),
    ("targetHandle" => TargetHandle(Primitive<Cow<'de, str>>)),
    ("transformOnFraction" => TransformOnFraction(Primitive<f32>)),
    ("normalOnFraction" => NormalOnFraction(Primitive<f32>)),
    ("fadeInDuration" => FadeInDuration(Primitive<f32>)),
    ("fadeOutDuration" => FadeOutDuration(Primitive<f32>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("handleChangeSpeed" => HandleChangeSpeed(Primitive<f32>)),
    ("handleChangeMode" => HandleChangeMode(Primitive<HandleChangeMode>)),
    ("fixUp" => FixUp(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
