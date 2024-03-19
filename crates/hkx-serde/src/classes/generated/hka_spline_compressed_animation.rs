//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaSplineCompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 132
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x792ee0bb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSplineCompressedAnimation<'a> {
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AnimationType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AnimationType>),
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"numberOfTransformTracks"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfTransformTracks")]
    NumberOfTransformTracks(Primitive<i32>),
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"numberOfFloatTracks"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfFloatTracks")]
    NumberOfFloatTracks(Primitive<i32>),
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"extractedMotion"`
    /// -   type: `struct hkaAnimatedReferenceFrame*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkaAnimation` => parent: `hkReferencedObject`) field Info
    /// -   name:`"annotationTracks"`
    /// -   type: `hkArray<struct hkaAnnotationTrack>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotationTracks")]
    AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'a>>),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"numFrames"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numFrames")]
    NumFrames(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numBlocks"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBlocks")]
    NumBlocks(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxFramesPerBlock"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFramesPerBlock")]
    MaxFramesPerBlock(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maskAndQuantizationSize"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maskAndQuantizationSize")]
    MaskAndQuantizationSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockDuration"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockDuration")]
    BlockDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockInverseDuration"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockInverseDuration")]
    BlockInverseDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"frameDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameDuration")]
    FrameDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockOffsets")]
    BlockOffsets(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatBlockOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatBlockOffsets")]
    FloatBlockOffsets(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"transformOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformOffsets")]
    TransformOffsets(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatOffsets"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatOffsets")]
    FloatOffsets(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"endian"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endian")]
    Endian(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimation<'de>, "@name",
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("numFrames" => NumFrames(Primitive<i32>)),
    ("numBlocks" => NumBlocks(Primitive<i32>)),
    ("maxFramesPerBlock" => MaxFramesPerBlock(Primitive<i32>)),
    ("maskAndQuantizationSize" => MaskAndQuantizationSize(Primitive<i32>)),
    ("blockDuration" => BlockDuration(Primitive<f32>)),
    ("blockInverseDuration" => BlockInverseDuration(Primitive<f32>)),
    ("frameDuration" => FrameDuration(Primitive<f32>)),
    ("blockOffsets" => BlockOffsets(HkArrayNum<u32>)),
    ("floatBlockOffsets" => FloatBlockOffsets(HkArrayNum<u32>)),
    ("transformOffsets" => TransformOffsets(HkArrayNum<u32>)),
    ("floatOffsets" => FloatOffsets(HkArrayNum<u32>)),
    ("data" => Data(HkArrayNum<u8>)),
    ("endian" => Endian(Primitive<i32>)),
}
