//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendCurveUtils`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendCurveUtils {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendCurveUtils, "@name",
}

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
