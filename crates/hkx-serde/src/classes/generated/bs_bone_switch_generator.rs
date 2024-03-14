//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSBoneSwitchGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSBoneSwitchGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xf33d3eea`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsBoneSwitchGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"ChildrenA"`
    /// -   type: `hkArray&lt;BSBoneSwitchGeneratorBoneData*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ChildrenA")]
    ChildrenA(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsBoneSwitchGenerator<'de>, "@name",
    ("pDefaultGenerator" => PDefaultGenerator(Cow<'de, str>)),
    ("ChildrenA" => ChildrenA(HkArrayRef<Cow<'de, str>>)),
}
