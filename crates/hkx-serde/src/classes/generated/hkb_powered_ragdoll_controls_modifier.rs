//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbPoweredRagdollControlsModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbPoweredRagdollControlsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x7cb54065`
/// -   version: 5
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoweredRagdollControlsModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbPoweredRagdollControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbPoweredRagdollControlData),
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModelModeData"`
    /// -   type: `struct hkbWorldFromModelModeData`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelModeData")]
    WorldFromModelModeData(HkbWorldFromModelModeData),
    /// # C++ Class Fields Info
    /// -   name:`"boneWeights"`
    /// -   type: `struct hkbBoneWeightArray*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneWeights")]
    BoneWeights(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoweredRagdollControlsModifier<'de>, "@name",
    ("controlData" => ControlData(HkbPoweredRagdollControlData)),
    ("bones" => Bones(Cow<'de, str>)),
    ("worldFromModelModeData" => WorldFromModelModeData(HkbWorldFromModelModeData)),
    ("boneWeights" => BoneWeights(Cow<'de, str>)),
}
