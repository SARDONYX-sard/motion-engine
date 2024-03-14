//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbWorldFromModelModeData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbWorldFromModelModeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xa3af8783`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbWorldFromModelModeData {
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone0"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone0")]
    PoseMatchingBone0(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone1"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone1")]
    PoseMatchingBone1(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone2"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone2")]
    PoseMatchingBone2(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum WorldFromModelMode`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(Primitive<WorldFromModelMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbWorldFromModelModeData, "@name",
    ("poseMatchingBone0" => PoseMatchingBone0(Primitive<i16>)),
    ("poseMatchingBone1" => PoseMatchingBone1(Primitive<i16>)),
    ("poseMatchingBone2" => PoseMatchingBone2(Primitive<i16>)),
    ("mode" => Mode(Primitive<WorldFromModelMode>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldFromModelMode {
    #[serde(rename = "WORLD_FROM_MODEL_MODE_USE_OLD")]
    WorldFromModelModeUseOld = 0,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_USE_INPUT")]
    WorldFromModelModeUseInput = 1,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_COMPUTE")]
    WorldFromModelModeCompute = 2,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_NONE")]
    WorldFromModelModeNone = 3,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_RAGDOLL")]
    WorldFromModelModeRagdoll = 4,
}
