//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEvaluateHandleModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEvaluateHandleModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x79757102`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateHandleModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handle")]
    Handle(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"handlePositionOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handlePositionOut")]
    HandlePositionOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleRotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleRotationOut")]
    HandleRotationOut(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isValidOut"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isValidOut")]
    IsValidOut(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeSpeed")]
    HandleChangeSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeMode")]
    HandleChangeMode(Primitive<HandleChangeMode>),
    /// # C++ Class Fields Info
    /// -   name:`"oldHandle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldHandle", skip_serializing)]
    OldHandle(HkbHandle),
    /// # C++ Class Fields Info
    /// -   name:`"oldHandlePosition"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldHandlePosition", skip_serializing)]
    OldHandlePosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"oldHandleRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldHandleRotation", skip_serializing)]
    OldHandleRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"smoothlyChangingHandles"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "smoothlyChangingHandles", skip_serializing)]
    SmoothlyChangingHandles(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateHandleModifier<'de>, "@name",
    ("handle" => Handle(Primitive<Cow<'de, str>>)),
    ("handlePositionOut" => HandlePositionOut(Vector4<f32>)),
    ("handleRotationOut" => HandleRotationOut(Quaternion<f32>)),
    ("isValidOut" => IsValidOut(Primitive<bool>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("handleChangeSpeed" => HandleChangeSpeed(Primitive<f32>)),
    ("handleChangeMode" => HandleChangeMode(Primitive<HandleChangeMode>)),
    ("oldHandle" => OldHandle(HkbHandle)),
    ("oldHandlePosition" => OldHandlePosition(Vector4<f32>)),
    ("oldHandleRotation" => OldHandleRotation(Quaternion<f32>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
    ("smoothlyChangingHandles" => SmoothlyChangingHandles(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
