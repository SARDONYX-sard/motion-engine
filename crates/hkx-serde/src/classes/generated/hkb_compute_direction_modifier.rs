//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbComputeDirectionModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbComputeDirectionModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xdf358bd3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeDirectionModifier {
    /// # C++ Class Fields Info
    /// -   name:`"pointIn"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointIn")]
    PointIn(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pointOut"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointOut")]
    PointOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"groundAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundAngleOut")]
    GroundAngleOut(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"upAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upAngleOut")]
    UpAngleOut(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalOffset")]
    VerticalOffset(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"reverseGroundAngle"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseGroundAngle")]
    ReverseGroundAngle(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"reverseUpAngle"`
    /// -   type: `hkBool`
    /// - offset: 93
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseUpAngle")]
    ReverseUpAngle(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"projectPoint"`
    /// -   type: `hkBool`
    /// - offset: 94
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "projectPoint")]
    ProjectPoint(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"normalizePoint"`
    /// -   type: `hkBool`
    /// - offset: 95
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalizePoint")]
    NormalizePoint(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"computeOnlyOnce"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computeOnlyOnce")]
    ComputeOnlyOnce(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"computedOutput"`
    /// -   type: `hkBool`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computedOutput")]
    ComputedOutput(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeDirectionModifier, "@name",
    ("pointIn" => PointIn(Vector4<f32>)),
    ("pointOut" => PointOut(Vector4<f32>)),
    ("groundAngleOut" => GroundAngleOut(Primitive<f32>)),
    ("upAngleOut" => UpAngleOut(Primitive<f32>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("reverseGroundAngle" => ReverseGroundAngle(Primitive<bool>)),
    ("reverseUpAngle" => ReverseUpAngle(Primitive<bool>)),
    ("projectPoint" => ProjectPoint(Primitive<bool>)),
    ("normalizePoint" => NormalizePoint(Primitive<bool>)),
    ("computeOnlyOnce" => ComputeOnlyOnce(Primitive<bool>)),
    ("computedOutput" => ComputedOutput(Primitive<bool>)),
}
