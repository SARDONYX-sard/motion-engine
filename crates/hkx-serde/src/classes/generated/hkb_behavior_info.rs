//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBehaviorInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xf7645395`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"idToNamePairs"`
    /// -   type: `hkArray&lt;struct hkbBehaviorInfoIdToNamePair&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "idToNamePairs")]
    IdToNamePairs(HkArrayClass<HkbBehaviorInfoIdToNamePair>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorInfo<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("idToNamePairs" => IdToNamePairs(HkArrayClass<HkbBehaviorInfoIdToNamePair>)),
}
