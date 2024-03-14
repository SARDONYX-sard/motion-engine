//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbComputeRotationToTargetModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbComputeRotationToTargetModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x47665f1c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeRotationToTargetModifier {
    /// # C++ Class Fields Info
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationOut")]
    RotationOut(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentPosition"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPosition")]
    CurrentPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentRotation")]
    CurrentRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"localAxisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localAxisOfRotation")]
    LocalAxisOfRotation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"localFacingDirection"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFacingDirection")]
    LocalFacingDirection(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"resultIsDelta"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resultIsDelta")]
    ResultIsDelta(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeRotationToTargetModifier, "@name",
    ("rotationOut" => RotationOut(Quaternion<f32>)),
    ("targetPosition" => TargetPosition(Vector4<f32>)),
    ("currentPosition" => CurrentPosition(Vector4<f32>)),
    ("currentRotation" => CurrentRotation(Quaternion<f32>)),
    ("localAxisOfRotation" => LocalAxisOfRotation(Vector4<f32>)),
    ("localFacingDirection" => LocalFacingDirection(Vector4<f32>)),
    ("resultIsDelta" => ResultIsDelta(Primitive<bool>)),
}
