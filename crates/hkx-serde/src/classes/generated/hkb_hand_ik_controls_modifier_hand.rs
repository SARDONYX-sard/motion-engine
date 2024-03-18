//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkControlsModifierHand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbHandIkControlsModifierHand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x9c72e9e3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkControlsModifierHand<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbHandIkControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(SingleClass<HkbHandIkControlData<'a>>),
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
    HkbHandIkControlsModifierHand<'de>, "@name",
    ("controlData" => ControlData(SingleClass<HkbHandIkControlData<'de>>)),
    ("handIndex" => HandIndex(Primitive<i32>)),
    ("enable" => Enable(Primitive<bool>)),
}
