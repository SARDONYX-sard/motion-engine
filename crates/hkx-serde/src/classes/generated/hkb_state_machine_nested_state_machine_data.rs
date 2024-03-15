//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineNestedStateMachineData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineNestedStateMachineData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x7358f5da`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineNestedStateMachineData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"nestedStateMachine"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nestedStateMachine", skip_serializing)]
    NestedStateMachine(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineNestedStateMachineData<'de>, "@name",
    ("nestedStateMachine" => NestedStateMachine(Primitive<Cow<'de, str>>)),
    ("eventIdMap" => EventIdMap(Primitive<Cow<'de, str>>)),
}
