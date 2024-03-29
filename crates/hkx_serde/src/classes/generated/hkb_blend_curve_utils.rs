//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendCurveUtils`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
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
pub enum HkbBlendCurveUtils {
}

impl ByteDeSerialize for HkbBlendCurveUtils {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
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
