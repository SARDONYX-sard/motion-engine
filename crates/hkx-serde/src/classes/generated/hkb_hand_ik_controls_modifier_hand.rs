//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbHandIkControlsModifierHand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbHandIkControlsModifierHand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x9c72e9e3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkControlsModifierHand {
    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbHandIkControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbHandIkControlData),
    /// # C++ Class Fields Info
    /// -   name:`"handIndex"`
    /// -   type: `hkInt32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handIndex")]
    HandIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkControlsModifierHand, "@name",
    ("controlData" => ControlData(HkbHandIkControlData)),
    ("handIndex" => HandIndex(Primitive<i32>)),
    ("enable" => Enable(Primitive<bool>)),
}
