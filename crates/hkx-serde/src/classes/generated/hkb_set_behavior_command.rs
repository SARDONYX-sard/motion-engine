//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbSetBehaviorCommand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSetBehaviorCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xe18b74b9`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetBehaviorCommand<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"behavior"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behavior")]
    Behavior(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootGenerator")]
    RootGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"referencedBehaviors"`
    /// -   type: `hkArray&lt;hkbBehaviorGraph*&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencedBehaviors")]
    ReferencedBehaviors(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"startStateIndex"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateIndex")]
    StartStateIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"randomizeSimulation"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "randomizeSimulation")]
    RandomizeSimulation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetBehaviorCommand<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("behavior" => Behavior(Cow<'de, str>)),
    ("rootGenerator" => RootGenerator(Cow<'de, str>)),
    ("referencedBehaviors" => ReferencedBehaviors(HkArrayRef<Cow<'de, str>>)),
    ("startStateIndex" => StartStateIndex(Primitive<i32>)),
    ("randomizeSimulation" => RandomizeSimulation(Primitive<bool>)),
    ("padding" => Padding(Primitive<i32>)),
}
