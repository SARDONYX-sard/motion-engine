//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRigidBodyRagdollControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbRigidBodyRagdollControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x1e0bc068`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRigidBodyRagdollControlData {
    /// # C++ Class Fields Info
    /// -   name:`"keyFrameHierarchyControlData"`
    /// -   type: `struct hkaKeyFrameHierarchyUtilityControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "keyFrameHierarchyControlData", default)]
    KeyFrameHierarchyControlData(HkaKeyFrameHierarchyUtilityControlData),
    /// # C++ Class Fields Info
    /// -   name:`"durationToBlend"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "durationToBlend", default)]
    DurationToBlend(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRigidBodyRagdollControlData, "@name",
    ("keyFrameHierarchyControlData" => KeyFrameHierarchyControlData(HkaKeyFrameHierarchyUtilityControlData)),
    ("durationToBlend" => DurationToBlend(Primitive<f32>)),
}
