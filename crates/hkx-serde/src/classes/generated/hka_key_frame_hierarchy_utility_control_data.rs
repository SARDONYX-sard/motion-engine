//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaKeyFrameHierarchyUtilityControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaKeyFrameHierarchyUtilityControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xa3d0ac71`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaKeyFrameHierarchyUtilityControlData {
    /// # C++ Class Fields Info
    /// -   name:`"hierarchyGain"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hierarchyGain")]
    HierarchyGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"velocityDamping"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityDamping")]
    VelocityDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"accelerationGain"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "accelerationGain")]
    AccelerationGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"velocityGain"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityGain")]
    VelocityGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"positionGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionGain")]
    PositionGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"positionMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionMaxLinearVelocity")]
    PositionMaxLinearVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"positionMaxAngularVelocity"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionMaxAngularVelocity")]
    PositionMaxAngularVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapGain"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapGain")]
    SnapGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxLinearVelocity")]
    SnapMaxLinearVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxAngularVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxAngularVelocity")]
    SnapMaxAngularVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxLinearDistance"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxLinearDistance")]
    SnapMaxLinearDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxAngularDistance"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapMaxAngularDistance")]
    SnapMaxAngularDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaKeyFrameHierarchyUtilityControlData, "@name",
    ("hierarchyGain" => HierarchyGain(Primitive<f32>)),
    ("velocityDamping" => VelocityDamping(Primitive<f32>)),
    ("accelerationGain" => AccelerationGain(Primitive<f32>)),
    ("velocityGain" => VelocityGain(Primitive<f32>)),
    ("positionGain" => PositionGain(Primitive<f32>)),
    ("positionMaxLinearVelocity" => PositionMaxLinearVelocity(Primitive<f32>)),
    ("positionMaxAngularVelocity" => PositionMaxAngularVelocity(Primitive<f32>)),
    ("snapGain" => SnapGain(Primitive<f32>)),
    ("snapMaxLinearVelocity" => SnapMaxLinearVelocity(Primitive<f32>)),
    ("snapMaxAngularVelocity" => SnapMaxAngularVelocity(Primitive<f32>)),
    ("snapMaxLinearDistance" => SnapMaxLinearDistance(Primitive<f32>)),
    ("snapMaxAngularDistance" => SnapMaxAngularDistance(Primitive<f32>)),
}
