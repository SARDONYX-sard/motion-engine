//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSOffsetAnimationGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSOffsetAnimationGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xb8571122`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsOffsetAnimationGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"pOffsetClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pOffsetClipGenerator")]
    POffsetClipGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetVariable"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetVariable")]
    FOffsetVariable(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetRangeStart"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetRangeStart")]
    FOffsetRangeStart(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetRangeEnd"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetRangeEnd")]
    FOffsetRangeEnd(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"BoneOffsetA"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "BoneOffsetA", skip_serializing)]
    BoneOffsetA(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"BoneIndexA"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "BoneIndexA", skip_serializing)]
    BoneIndexA(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"fCurrentPercentage"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fCurrentPercentage", skip_serializing)]
    FCurrentPercentage(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"iCurrentFrame"`
    /// -   type: `hkUint32`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "iCurrentFrame", skip_serializing)]
    ICurrentFrame(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"bZeroOffset"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bZeroOffset", skip_serializing)]
    BZeroOffset(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bOffsetValid"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bOffsetValid", skip_serializing)]
    BOffsetValid(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsOffsetAnimationGenerator<'de>, "@name",
    ("pDefaultGenerator" => PDefaultGenerator(Cow<'de, str>)),
    ("pOffsetClipGenerator" => POffsetClipGenerator(Cow<'de, str>)),
    ("fOffsetVariable" => FOffsetVariable(Primitive<f32>)),
    ("fOffsetRangeStart" => FOffsetRangeStart(Primitive<f32>)),
    ("fOffsetRangeEnd" => FOffsetRangeEnd(Primitive<f32>)),
    ("BoneOffsetA" => BoneOffsetA(HkArrayRef<()>)),
    ("BoneIndexA" => BoneIndexA(HkArrayRef<()>)),
    ("fCurrentPercentage" => FCurrentPercentage(Primitive<f32>)),
    ("iCurrentFrame" => ICurrentFrame(Primitive<u32>)),
    ("bZeroOffset" => BZeroOffset(Primitive<bool>)),
    ("bOffsetValid" => BOffsetValid(Primitive<bool>)),
}
