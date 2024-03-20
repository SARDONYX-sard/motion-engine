//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendCurveUtils`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbBlendCurveUtils`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x23041af0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@name")]
pub enum HkbBlendCurveUtils {}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlendCurve {
    #[serde(rename = "BLEND_CURVE_SMOOTH")]
    BlendCurveSmooth = 0,
    #[serde(rename = "BLEND_CURVE_LINEAR")]
    BlendCurveLinear = 1,
    #[serde(rename = "BLEND_CURVE_LINEAR_TO_SMOOTH")]
    BlendCurveLinearToSmooth = 2,
    #[serde(rename = "BLEND_CURVE_SMOOTH_TO_LINEAR")]
    BlendCurveSmoothToLinear = 3,
}
