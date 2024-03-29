//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaQuantizedAnimationTrackCompressionParams`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkaQuantizedAnimationTrackCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xf7d64649`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaQuantizedAnimationTrackCompressionParams {
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
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaQuantizedAnimationTrackCompressionParams, "@name",
    ("rotationTolerance" => RotationTolerance(Primitive<f32>)),
    ("translationTolerance" => TranslationTolerance(Primitive<f32>)),
    ("scaleTolerance" => ScaleTolerance(Primitive<f32>)),
    ("floatingTolerance" => FloatingTolerance(Primitive<f32>)),
}

impl ByteDeSerialize for HkaQuantizedAnimationTrackCompressionParams {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
