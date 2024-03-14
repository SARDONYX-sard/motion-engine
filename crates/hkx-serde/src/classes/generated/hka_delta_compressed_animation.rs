//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaDeltaCompressedAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaDeltaCompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 120
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x90a68d40`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaDeltaCompressedAnimation {
    /// # C++ Class Fields Info
    /// -   name:`"numberOfPoses"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfPoses")]
    NumberOfPoses(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockSize"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockSize")]
    BlockSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"qFormat"`
    /// -   type: `struct hkaDeltaCompressedAnimationQuantizationFormat`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "qFormat")]
    QFormat(HkaDeltaCompressedAnimationQuantizationFormat),
    /// # C++ Class Fields Info
    /// -   name:`"quantizedDataIdx"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataIdx")]
    QuantizedDataIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"quantizedDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataSize")]
    QuantizedDataSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticMaskIdx"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticMaskIdx")]
    StaticMaskIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticMaskSize"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticMaskSize")]
    StaticMaskSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticDOFsIdx"`
    /// -   type: `hkUint32`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticDOFsIdx")]
    StaticDoFsIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticDOFsSize"`
    /// -   type: `hkUint32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticDOFsSize")]
    StaticDoFsSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numStaticTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numStaticTransformDOFs")]
    NumStaticTransformDoFs(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numDynamicTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numDynamicTransformDOFs")]
    NumDynamicTransformDoFs(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"totalBlockSize"`
    /// -   type: `hkUint32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "totalBlockSize")]
    TotalBlockSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"lastBlockSize"`
    /// -   type: `hkUint32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastBlockSize")]
    LastBlockSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"dataBuffer"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dataBuffer")]
    DataBuffer(HkArrayRef<Primitive<u8>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaDeltaCompressedAnimation, "@name",
    ("numberOfPoses" => NumberOfPoses(Primitive<i32>)),
    ("blockSize" => BlockSize(Primitive<i32>)),
    ("qFormat" => QFormat(HkaDeltaCompressedAnimationQuantizationFormat)),
    ("quantizedDataIdx" => QuantizedDataIdx(Primitive<u32>)),
    ("quantizedDataSize" => QuantizedDataSize(Primitive<u32>)),
    ("staticMaskIdx" => StaticMaskIdx(Primitive<u32>)),
    ("staticMaskSize" => StaticMaskSize(Primitive<u32>)),
    ("staticDOFsIdx" => StaticDoFsIdx(Primitive<u32>)),
    ("staticDOFsSize" => StaticDoFsSize(Primitive<u32>)),
    ("numStaticTransformDOFs" => NumStaticTransformDoFs(Primitive<u32>)),
    ("numDynamicTransformDOFs" => NumDynamicTransformDoFs(Primitive<u32>)),
    ("totalBlockSize" => TotalBlockSize(Primitive<u32>)),
    ("lastBlockSize" => LastBlockSize(Primitive<u32>)),
    ("dataBuffer" => DataBuffer(HkArrayRef<Primitive<u8>>)),
}
