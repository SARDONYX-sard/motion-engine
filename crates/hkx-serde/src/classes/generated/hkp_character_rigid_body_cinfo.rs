//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCharacterRigidBodyCinfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCharacterRigidBodyCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpCharacterControllerCinfo`/`0xda8c7d7d`
/// - signature: `0x892f441`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCharacterRigidBodyCinfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"unweldingHeightOffsetFactor"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "unweldingHeightOffsetFactor")]
    UnweldingHeightOffsetFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSpeedForSimplexSolver"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSpeedForSimplexSolver")]
    MaxSpeedForSimplexSolver(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"supportDistance"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "supportDistance")]
    SupportDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"hardSupportDistance"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hardSupportDistance")]
    HardSupportDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vdbColor"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vdbColor")]
    VdbColor(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCharacterRigidBodyCinfo<'de>, "@name",
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("position" => Position(Vector4<f32>)),
    ("rotation" => Rotation(Quaternion<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("maxLinearVelocity" => MaxLinearVelocity(Primitive<f32>)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(Primitive<f32>)),
    ("up" => Up(Vector4<f32>)),
    ("maxSlope" => MaxSlope(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("unweldingHeightOffsetFactor" => UnweldingHeightOffsetFactor(Primitive<f32>)),
    ("maxSpeedForSimplexSolver" => MaxSpeedForSimplexSolver(Primitive<f32>)),
    ("supportDistance" => SupportDistance(Primitive<f32>)),
    ("hardSupportDistance" => HardSupportDistance(Primitive<f32>)),
    ("vdbColor" => VdbColor(Primitive<i32>)),
}
