//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPackfileHeader`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkPackfileHeader`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x79f9ffda`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPackfileHeader {
    /// # C++ Class Fields Info
    /// -   name:`"magic"`
    /// -   type: `hkInt32[2]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "magic", default)]
    Magic([Primitive<i32>; 2]),
    /// # C++ Class Fields Info
    /// -   name:`"userTag"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userTag", default)]
    UserTag(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"fileVersion"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fileVersion", default)]
    FileVersion(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"layoutRules"`
    /// -   type: `hkUint8[4]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "layoutRules", default)]
    LayoutRules([Primitive<u8>; 4]),
    /// # C++ Class Fields Info
    /// -   name:`"numSections"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSections", default)]
    NumSections(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contentsSectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsSectionIndex", default)]
    ContentsSectionIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contentsSectionOffset"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsSectionOffset", default)]
    ContentsSectionOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contentsClassNameSectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsClassNameSectionIndex", default)]
    ContentsClassNameSectionIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contentsClassNameSectionOffset"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsClassNameSectionOffset", default)]
    ContentsClassNameSectionOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contentsVersion"`
    /// -   type: `hkChar[16]`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contentsVersion", default)]
    ContentsVersion([Primitive<char>; 16]),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags", default)]
    Flags(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"pad"`
    /// -   type: `hkInt32[1]`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad", default)]
    Pad([Primitive<i32>; 1]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPackfileHeader, "@name",
    ("magic" => Magic([Primitive<i32>; 2])),
    ("userTag" => UserTag(Primitive<i32>)),
    ("fileVersion" => FileVersion(Primitive<i32>)),
    ("layoutRules" => LayoutRules([Primitive<u8>; 4])),
    ("numSections" => NumSections(Primitive<i32>)),
    ("contentsSectionIndex" => ContentsSectionIndex(Primitive<i32>)),
    ("contentsSectionOffset" => ContentsSectionOffset(Primitive<i32>)),
    ("contentsClassNameSectionIndex" => ContentsClassNameSectionIndex(Primitive<i32>)),
    ("contentsClassNameSectionOffset" => ContentsClassNameSectionOffset(Primitive<i32>)),
    ("contentsVersion" => ContentsVersion([Primitive<char>; 16])),
    ("flags" => Flags(Primitive<i32>)),
    ("pad" => Pad([Primitive<i32>; 1])),
}
