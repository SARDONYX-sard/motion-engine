//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbExtractRagdollPoseModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbExtractRagdollPoseModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x804dcbab`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExtractRagdollPoseModifier {
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone0"`
    /// -   type: `hkInt16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone0")]
    PoseMatchingBone0(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone1"`
    /// -   type: `hkInt16`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone1")]
    PoseMatchingBone1(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone2"`
    /// -   type: `hkInt16`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseMatchingBone2")]
    PoseMatchingBone2(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"enableComputeWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableComputeWorldFromModel")]
    EnableComputeWorldFromModel(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExtractRagdollPoseModifier, "@name",
    ("poseMatchingBone0" => PoseMatchingBone0(Primitive<i16>)),
    ("poseMatchingBone1" => PoseMatchingBone1(Primitive<i16>)),
    ("poseMatchingBone2" => PoseMatchingBone2(Primitive<i16>)),
    ("enableComputeWorldFromModel" => EnableComputeWorldFromModel(Primitive<bool>)),
}
