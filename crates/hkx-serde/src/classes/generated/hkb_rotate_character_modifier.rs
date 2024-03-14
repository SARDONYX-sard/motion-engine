//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRotateCharacterModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbRotateCharacterModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x877ebc0b`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRotateCharacterModifier {
    /// # C++ Class Fields Info
    /// -   name:`"degreesPerSecond"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "degreesPerSecond")]
    DegreesPerSecond(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"speedMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "speedMultiplier")]
    SpeedMultiplier(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"axisOfRotation"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axisOfRotation")]
    AxisOfRotation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angle"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "angle", skip_serializing)]
    Angle(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRotateCharacterModifier, "@name",
    ("degreesPerSecond" => DegreesPerSecond(Primitive<f32>)),
    ("speedMultiplier" => SpeedMultiplier(Primitive<f32>)),
    ("axisOfRotation" => AxisOfRotation(Vector4<f32>)),
    ("angle" => Angle(Primitive<f32>)),
}
