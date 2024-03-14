//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSBoneSwitchGeneratorBoneData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSBoneSwitchGeneratorBoneData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xc1215be6`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsBoneSwitchGeneratorBoneData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pGenerator")]
    PGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"spBoneWeight"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spBoneWeight")]
    SpBoneWeight(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsBoneSwitchGeneratorBoneData<'de>, "@name",
    ("pGenerator" => PGenerator(Cow<'de, str>)),
    ("spBoneWeight" => SpBoneWeight(Cow<'de, str>)),
}
