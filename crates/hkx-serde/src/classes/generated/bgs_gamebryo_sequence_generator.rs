//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BGSGamebryoSequenceGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BGSGamebryoSequenceGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xc8df2d77`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BgsGamebryoSequenceGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pSequence"`
    /// -   type: `char*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pSequence")]
    PSequence(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eBlendModeFunction"`
    /// -   type: `enum BlendModeFunction`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eBlendModeFunction")]
    EBlendModeFunction(Primitive<BlendModeFunction>),
    /// # C++ Class Fields Info
    /// -   name:`"fPercent"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fPercent")]
    FPercent(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"events"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "events", skip_serializing)]
    Events(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"fTime"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fTime", skip_serializing)]
    FTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bDelayedActivate"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bDelayedActivate", skip_serializing)]
    BDelayedActivate(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bLooping"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bLooping", skip_serializing)]
    BLooping(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BgsGamebryoSequenceGenerator<'de>, "@name",
    ("pSequence" => PSequence(Primitive<Cow<'de, str>>)),
    ("eBlendModeFunction" => EBlendModeFunction(Primitive<BlendModeFunction>)),
    ("fPercent" => FPercent(Primitive<f32>)),
    ("events" => Events(HkArrayRef<Primitive<()>>)),
    ("fTime" => FTime(Primitive<f32>)),
    ("bDelayedActivate" => BDelayedActivate(Primitive<bool>)),
    ("bLooping" => BLooping(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlendModeFunction {
    #[serde(rename = "BMF_NONE")]
    BmfNone = 0,
    #[serde(rename = "BMF_PERCENT")]
    BmfPercent = 1,
    #[serde(rename = "BMF_ONE_MINUS_PERCENT")]
    BmfOneMinusPercent = 2,
}
