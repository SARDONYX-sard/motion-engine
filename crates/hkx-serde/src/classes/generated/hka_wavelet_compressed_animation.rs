//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaWaveletCompressedAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaWaveletCompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x77cf0962`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaWaveletCompressedAnimation {
    /// # C++ Parent class(`hkaAnimation`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AnimationType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<AnimationType>),
    /// # C++ Parent class(`hkaAnimation`, parent: `hkReferencedObject`) field Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration", default)]
    Duration(Primitive<f32>),
    /// # C++ Parent class(`hkaAnimation`, parent: `hkReferencedObject`) field Info
    /// -   name:`"numberOfTransformTracks"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfTransformTracks", default)]
    NumberOfTransformTracks(Primitive<i32>),
    /// # C++ Parent class(`hkaAnimation`, parent: `hkReferencedObject`) field Info
    /// -   name:`"numberOfFloatTracks"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfFloatTracks", default)]
    NumberOfFloatTracks(Primitive<i32>),
    /// # C++ Parent class(`hkaAnimation`, parent: `hkReferencedObject`) field Info
    /// -   name:`"extractedMotion"`
    /// -   type: `struct hkaAnimatedReferenceFrame*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotion", default)]
    ExtractedMotion(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkaAnimation`, parent: `hkReferencedObject`) field Info
    /// -   name:`"annotationTracks"`
    /// -   type: `hkArray&lt;struct hkaAnnotationTrack&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotationTracks", default)]
    AnnotationTracks(HkArrayClass<HkaAnnotationTrack>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"numberOfPoses"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfPoses", default)]
    NumberOfPoses(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockSize"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockSize", default)]
    BlockSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"qFormat"`
    /// -   type: `struct hkaWaveletCompressedAnimationQuantizationFormat`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "qFormat", default)]
    QFormat(HkaWaveletCompressedAnimationQuantizationFormat),
    /// # C++ Class Fields Info
    /// -   name:`"staticMaskIdx"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticMaskIdx", default)]
    StaticMaskIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticDOFsIdx"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticDOFsIdx", default)]
    StaticDoFsIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numStaticTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numStaticTransformDOFs", default)]
    NumStaticTransformDoFs(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numDynamicTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numDynamicTransformDOFs", default)]
    NumDynamicTransformDoFs(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockIndexIdx"`
    /// -   type: `hkUint32`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockIndexIdx", default)]
    BlockIndexIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockIndexSize"`
    /// -   type: `hkUint32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockIndexSize", default)]
    BlockIndexSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"quantizedDataIdx"`
    /// -   type: `hkUint32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataIdx", default)]
    QuantizedDataIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"quantizedDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataSize", default)]
    QuantizedDataSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"dataBuffer"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dataBuffer", default)]
    DataBuffer(HkArrayRef<Primitive<u8>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimation, "@name",
    ("type" => Type(Primitive<AnimationType>)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Primitive<Cow<'de, str>>)),
    ("annotationTracks" => AnnotationTracks(HkArrayClass<HkaAnnotationTrack>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("numberOfPoses" => NumberOfPoses(Primitive<i32>)),
    ("blockSize" => BlockSize(Primitive<i32>)),
    ("qFormat" => QFormat(HkaWaveletCompressedAnimationQuantizationFormat)),
    ("staticMaskIdx" => StaticMaskIdx(Primitive<u32>)),
    ("staticDOFsIdx" => StaticDoFsIdx(Primitive<u32>)),
    ("numStaticTransformDOFs" => NumStaticTransformDoFs(Primitive<u32>)),
    ("numDynamicTransformDOFs" => NumDynamicTransformDoFs(Primitive<u32>)),
    ("blockIndexIdx" => BlockIndexIdx(Primitive<u32>)),
    ("blockIndexSize" => BlockIndexSize(Primitive<u32>)),
    ("quantizedDataIdx" => QuantizedDataIdx(Primitive<u32>)),
    ("quantizedDataSize" => QuantizedDataSize(Primitive<u32>)),
    ("dataBuffer" => DataBuffer(HkArrayRef<Primitive<u8>>)),
}
