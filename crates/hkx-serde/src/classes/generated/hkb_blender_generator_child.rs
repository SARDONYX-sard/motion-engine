//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbBlenderGeneratorChild`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBlenderGeneratorChild`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xe2b384b0`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlenderGeneratorChild<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "generator")]
    Generator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"boneWeights"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneWeights")]
    BoneWeights(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"weight"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weight")]
    Weight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModelWeight"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelWeight")]
    WorldFromModelWeight(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorChild<'de>, "@name",
    ("generator" => Generator(Cow<'de, str>)),
    ("boneWeights" => BoneWeights(Cow<'de, str>)),
    ("weight" => Weight(Primitive<f32>)),
    ("worldFromModelWeight" => WorldFromModelWeight(Primitive<f32>)),
}
