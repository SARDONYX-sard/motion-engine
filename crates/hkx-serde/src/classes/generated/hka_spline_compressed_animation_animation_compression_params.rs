//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimationAnimationCompressionParams`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkaSplineCompressedAnimationAnimationCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0xde830789`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSplineCompressedAnimationAnimationCompressionParams {
    /// # C++ Class Fields Info
    /// -   name:`"maxFramesPerBlock"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFramesPerBlock")]
    MaxFramesPerBlock(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"enableSampleSingleTracks"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableSampleSingleTracks")]
    EnableSampleSingleTracks(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationAnimationCompressionParams, "@name",
    ("maxFramesPerBlock" => MaxFramesPerBlock(Primitive<u16>)),
    ("enableSampleSingleTracks" => EnableSampleSingleTracks(Primitive<bool>)),
}

impl ByteDeSerialize for HkaSplineCompressedAnimationAnimationCompressionParams {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
