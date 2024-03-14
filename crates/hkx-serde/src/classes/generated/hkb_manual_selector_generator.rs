//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbManualSelectorGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbManualSelectorGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xd932fab8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbManualSelectorGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"generators"`
    /// -   type: `hkArray&lt;hkbGenerator*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generators")]
    Generators(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"selectedGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectedGeneratorIndex")]
    SelectedGeneratorIndex(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"currentGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentGeneratorIndex")]
    CurrentGeneratorIndex(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbManualSelectorGenerator<'de>, "@name",
    ("generators" => Generators(HkArrayRef<Cow<'de, str>>)),
    ("selectedGeneratorIndex" => SelectedGeneratorIndex(Primitive<i8>)),
    ("currentGeneratorIndex" => CurrentGeneratorIndex(Primitive<i8>)),
}
