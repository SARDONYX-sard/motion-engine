//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRigidBodyRagdollControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRigidBodyRagdollControlData {
    /// # C++ Class Fields Info
    /// -   name:`"keyFrameHierarchyControlData"`
    /// -   type: `struct hkaKeyFrameHierarchyUtilityControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "keyFrameHierarchyControlData")]
    KeyFrameHierarchyControlData(SingleClass<HkaKeyFrameHierarchyUtilityControlData>),
    /// # C++ Class Fields Info
    /// -   name:`"durationToBlend"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "durationToBlend")]
    DurationToBlend(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRigidBodyRagdollControlData, "@name",
    ("keyFrameHierarchyControlData" => KeyFrameHierarchyControlData(SingleClass<HkaKeyFrameHierarchyUtilityControlData>)),
    ("durationToBlend" => DurationToBlend(Primitive<f32>)),
}
