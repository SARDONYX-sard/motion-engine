//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSplineCompressedAnimation {
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
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockOffsets")]
    BlockOffsets(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"floatBlockOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatBlockOffsets")]
    FloatBlockOffsets(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"transformOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformOffsets")]
    TransformOffsets(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"floatOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatOffsets")]
    FloatOffsets(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayRef<Primitive<u8>>),
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
    HkaSplineCompressedAnimation, "@name",
    ("numFrames" => NumFrames(Primitive<i32>)),
    ("numBlocks" => NumBlocks(Primitive<i32>)),
    ("maxFramesPerBlock" => MaxFramesPerBlock(Primitive<i32>)),
    ("maskAndQuantizationSize" => MaskAndQuantizationSize(Primitive<i32>)),
    ("blockDuration" => BlockDuration(Primitive<f32>)),
    ("blockInverseDuration" => BlockInverseDuration(Primitive<f32>)),
    ("frameDuration" => FrameDuration(Primitive<f32>)),
    ("blockOffsets" => BlockOffsets(HkArrayRef<Primitive<u32>>)),
    ("floatBlockOffsets" => FloatBlockOffsets(HkArrayRef<Primitive<u32>>)),
    ("transformOffsets" => TransformOffsets(HkArrayRef<Primitive<u32>>)),
    ("floatOffsets" => FloatOffsets(HkArrayRef<Primitive<u32>>)),
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
    ("endian" => Endian(Primitive<i32>)),
}
