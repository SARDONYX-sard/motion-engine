//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa6fa7e88`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnimation<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum AnimationType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(AnimationType),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numberOfTransformTracks"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfTransformTracks")]
    NumberOfTransformTracks(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numberOfFloatTracks"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfFloatTracks")]
    NumberOfFloatTracks(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"extractedMotion"`
    /// -   type: `struct hkaAnimatedReferenceFrame*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"annotationTracks"`
    /// -   type: `hkArray&lt;struct hkaAnnotationTrack&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotationTracks")]
    AnnotationTracks(HkArrayClass<HkaAnnotationTrack>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimation<'de>, "@name",
    ("type" => Type(AnimationType)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Cow<'de, str>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationType {
    #[serde(rename = "HK_UNKNOWN_ANIMATION")]
    HkUnknownAnimation = 0,
    #[serde(rename = "HK_INTERLEAVED_ANIMATION")]
    HkInterleavedAnimation = 1,
    #[serde(rename = "HK_DELTA_COMPRESSED_ANIMATION")]
    HkDeltaCompressedAnimation = 2,
    #[serde(rename = "HK_WAVELET_COMPRESSED_ANIMATION")]
    HkWaveletCompressedAnimation = 3,
    #[serde(rename = "HK_MIRRORED_ANIMATION")]
    HkMirroredAnimation = 4,
    #[serde(rename = "HK_SPLINE_COMPRESSED_ANIMATION")]
    HkSplineCompressedAnimation = 5,
    #[serde(rename = "HK_QUANTIZED_COMPRESSED_ANIMATION")]
    HkQuantizedCompressedAnimation = 6,
}
