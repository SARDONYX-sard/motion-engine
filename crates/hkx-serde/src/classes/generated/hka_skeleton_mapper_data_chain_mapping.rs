//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonMapperDataChainMapping`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkaSkeletonMapperDataChainMapping`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0xa528f7cf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonMapperDataChainMapping {
    /// # C++ Class Fields Info
    /// -   name:`"startBoneA"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneA")]
    StartBoneA(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endBoneA"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneA")]
    EndBoneA(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"startBoneB"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startBoneB")]
    StartBoneB(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endBoneB"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endBoneB")]
    EndBoneB(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"startAFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startAFromBTransform")]
    StartAFromBTransform(Primitive<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"endAFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endAFromBTransform")]
    EndAFromBTransform(Primitive<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataChainMapping, "@name",
    ("startBoneA" => StartBoneA(Primitive<i16>)),
    ("endBoneA" => EndBoneA(Primitive<i16>)),
    ("startBoneB" => StartBoneB(Primitive<i16>)),
    ("endBoneB" => EndBoneB(Primitive<i16>)),
    ("startAFromBTransform" => StartAFromBTransform(Primitive<QsTransform<f32>>)),
    ("endAFromBTransform" => EndAFromBTransform(Primitive<QsTransform<f32>>)),
}

impl ByteDeSerialize for HkaSkeletonMapperDataChainMapping {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
