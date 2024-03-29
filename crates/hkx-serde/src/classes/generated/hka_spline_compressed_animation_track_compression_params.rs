//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimationTrackCompressionParams`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkaSplineCompressedAnimationTrackCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: false
/// - signature: `0x42e878d3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSplineCompressedAnimationTrackCompressionParams {
    /// # C++ Class Fields Info
    /// -   name:`"rotationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationTolerance")]
    RotationTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"translationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationTolerance")]
    TranslationTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleTolerance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleTolerance")]
    ScaleTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatingTolerance"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatingTolerance")]
    FloatingTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationDegree"`
    /// -   type: `hkUint16`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationDegree")]
    RotationDegree(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"translationDegree"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationDegree")]
    TranslationDegree(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleDegree"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleDegree")]
    ScaleDegree(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"floatingDegree"`
    /// -   type: `hkUint16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatingDegree")]
    FloatingDegree(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationQuantizationType"`
    /// -   type: `enum RotationQuantization`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationQuantizationType")]
    RotationQuantizationType(Primitive<RotationQuantization>),
    /// # C++ Class Fields Info
    /// -   name:`"translationQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationQuantizationType")]
    TranslationQuantizationType(Primitive<ScalarQuantization>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleQuantizationType")]
    ScaleQuantizationType(Primitive<ScalarQuantization>),
    /// # C++ Class Fields Info
    /// -   name:`"floatQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatQuantizationType")]
    FloatQuantizationType(Primitive<ScalarQuantization>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationTrackCompressionParams, "@name",
    ("rotationTolerance" => RotationTolerance(Primitive<f32>)),
    ("translationTolerance" => TranslationTolerance(Primitive<f32>)),
    ("scaleTolerance" => ScaleTolerance(Primitive<f32>)),
    ("floatingTolerance" => FloatingTolerance(Primitive<f32>)),
    ("rotationDegree" => RotationDegree(Primitive<u16>)),
    ("translationDegree" => TranslationDegree(Primitive<u16>)),
    ("scaleDegree" => ScaleDegree(Primitive<u16>)),
    ("floatingDegree" => FloatingDegree(Primitive<u16>)),
    ("rotationQuantizationType" => RotationQuantizationType(Primitive<RotationQuantization>)),
    ("translationQuantizationType" => TranslationQuantizationType(Primitive<ScalarQuantization>)),
    ("scaleQuantizationType" => ScaleQuantizationType(Primitive<ScalarQuantization>)),
    ("floatQuantizationType" => FloatQuantizationType(Primitive<ScalarQuantization>)),
}

impl ByteDeSerialize for HkaSplineCompressedAnimationTrackCompressionParams {
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
pub enum RotationQuantization {
    #[serde(rename = "POLAR32")]
    Polar32 = 0,
    #[serde(rename = "THREECOMP40")]
    Threecomp40 = 1,
    #[serde(rename = "THREECOMP48")]
    Threecomp48 = 2,
    #[serde(rename = "THREECOMP24")]
    Threecomp24 = 3,
    #[serde(rename = "STRAIGHT16")]
    Straight16 = 4,
    #[serde(rename = "UNCOMPRESSED")]
    Uncompressed = 5,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ScalarQuantization {
    #[serde(rename = "BITS8")]
    Bits8 = 0,
    #[serde(rename = "BITS16")]
    Bits16 = 1,
}
