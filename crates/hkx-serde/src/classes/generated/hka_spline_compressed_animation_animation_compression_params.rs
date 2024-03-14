//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaSplineCompressedAnimationAnimationCompressionParams`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaSplineCompressedAnimationAnimationCompressionParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xde830789`
/// -   version: 0
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
