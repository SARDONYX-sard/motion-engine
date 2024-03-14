//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbNode`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbNode`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0x6d26f61d`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbNode<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Unknown),
    /// # C++ Class Fields Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode([Primitive<bool>; 1]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbNode<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Unknown)),
    ("padNode" => PadNode([Primitive<bool>; 1])),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetChildrenFlagBits {
    #[serde(rename = "FLAG_ACTIVE_ONLY")]
    FlagActiveOnly = 1,
    #[serde(rename = "FLAG_GENERATORS_ONLY")]
    FlagGeneratorsOnly = 2,
    #[serde(rename = "FLAG_IGNORE_REFERENCED_BEHAVIORS")]
    FlagIgnoreReferencedBehaviors = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloneState {
    #[serde(rename = "CLONE_STATE_DEFAULT")]
    CloneStateDefault = 0,
    #[serde(rename = "CLONE_STATE_TEMPLATE")]
    CloneStateTemplate = 1,
    #[serde(rename = "CLONE_STATE_CLONE")]
    CloneStateClone = 2,
    #[serde(rename = "CLONE_STATE_SHARABLE")]
    CloneStateSharable = 3,
}
