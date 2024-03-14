//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbStateMachineTransitionInfoArray`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineTransitionInfoArray`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xe397b11e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTransitionInfoArray {
    /// # C++ Class Fields Info
    /// -   name:`"transitions"`
    /// -   type: `hkArray&lt;struct hkbStateMachineTransitionInfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitions")]
    Transitions(HkArrayClass<HkbStateMachineTransitionInfo>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfoArray, "@name",
    ("transitions" => Transitions(HkArrayClass<HkbStateMachineTransitionInfo>)),
}
