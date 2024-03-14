//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbDelayedModifierInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbDelayedModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x85fb0b80`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDelayedModifierInternalState {
    /// # C++ Class Fields Info
    /// -   name:`"secondsElapsed"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "secondsElapsed")]
    SecondsElapsed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isActive")]
    IsActive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDelayedModifierInternalState, "@name",
    ("secondsElapsed" => SecondsElapsed(Primitive<f32>)),
    ("isActive" => IsActive(Primitive<bool>)),
}
