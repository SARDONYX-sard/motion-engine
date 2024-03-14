//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbComputeRotationFromAxisAngleModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbComputeRotationFromAxisAngleModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x9b3f6936`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeRotationFromAxisAngleModifier {
    /// # C++ Class Fields Info
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationOut")]
    RotationOut(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"axis"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axis")]
    Axis(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angleDegrees")]
    AngleDegrees(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeRotationFromAxisAngleModifier, "@name",
    ("rotationOut" => RotationOut(Quaternion<f32>)),
    ("axis" => Axis(Vector4<f32>)),
    ("angleDegrees" => AngleDegrees(Primitive<f32>)),
}
