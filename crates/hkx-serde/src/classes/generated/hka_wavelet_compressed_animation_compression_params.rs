//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaWaveletCompressedAnimationCompressionParams`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaWaveletCompressedAnimationCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: false
/// - signature: `0x27c6cafa`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaWaveletCompressedAnimationCompressionParams {
    /// # C++ Class Fields Info
    /// -   name:`"quantizationBits"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizationBits")]
    QuantizationBits(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"blockSize"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockSize")]
    BlockSize(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"preserve"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "preserve")]
    Preserve(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"truncProp"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "truncProp")]
    TruncProp(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"useOldStyleTruncation"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useOldStyleTruncation")]
    UseOldStyleTruncation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"absolutePositionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absolutePositionTolerance")]
    AbsolutePositionTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"relativePositionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativePositionTolerance")]
    RelativePositionTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationTolerance")]
    RotationTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleTolerance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleTolerance")]
    ScaleTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"absoluteFloatTolerance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absoluteFloatTolerance")]
    AbsoluteFloatTolerance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimationCompressionParams, "@name",
    ("quantizationBits" => QuantizationBits(Primitive<u16>)),
    ("blockSize" => BlockSize(Primitive<u16>)),
    ("preserve" => Preserve(Primitive<u16>)),
    ("truncProp" => TruncProp(Primitive<f32>)),
    ("useOldStyleTruncation" => UseOldStyleTruncation(Primitive<bool>)),
    ("absolutePositionTolerance" => AbsolutePositionTolerance(Primitive<f32>)),
    ("relativePositionTolerance" => RelativePositionTolerance(Primitive<f32>)),
    ("rotationTolerance" => RotationTolerance(Primitive<f32>)),
    ("scaleTolerance" => ScaleTolerance(Primitive<f32>)),
    ("absoluteFloatTolerance" => AbsoluteFloatTolerance(Primitive<f32>)),
}
