//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineTransitionInfoReference`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineTransitionInfoReference`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 6
/// -    vtable: false
/// - signature: `0x9810c2d0`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTransitionInfoReference {
    /// # C++ Class Fields Info
    /// -   name:`"fromStateIndex"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fromStateIndex")]
    FromStateIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionIndex"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionIndex")]
    TransitionIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"stateMachineId"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateMachineId")]
    StateMachineId(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfoReference, "@name",
    ("fromStateIndex" => FromStateIndex(Primitive<i16>)),
    ("transitionIndex" => TransitionIndex(Primitive<i16>)),
    ("stateMachineId" => StateMachineId(Primitive<i16>)),
}
