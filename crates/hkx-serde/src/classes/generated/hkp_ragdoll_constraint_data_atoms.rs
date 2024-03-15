//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpRagdollConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 336
/// -    vtable: false
/// - signature: `0xeed76b00`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkpSetLocalTransformsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setupStabilization")]
    SetupStabilization(HkpSetupStabilizationAtom),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollMotors"`
    /// -   type: `struct hkpRagdollMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollMotors")]
    RagdollMotors(HkpRagdollMotorConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angFriction")]
    AngFriction(HkpAngFrictionConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistLimit")]
    TwistLimit(HkpTwistLimitConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "coneLimit")]
    ConeLimit(HkpConeLimitConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 292
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planesLimit")]
    PlanesLimit(HkpConeLimitConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(HkpBallSocketConstraintAtom),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollConstraintDataAtoms, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("setupStabilization" => SetupStabilization(HkpSetupStabilizationAtom)),
    ("ragdollMotors" => RagdollMotors(HkpRagdollMotorConstraintAtom)),
    ("angFriction" => AngFriction(HkpAngFrictionConstraintAtom)),
    ("twistLimit" => TwistLimit(HkpTwistLimitConstraintAtom)),
    ("coneLimit" => ConeLimit(HkpConeLimitConstraintAtom)),
    ("planesLimit" => PlanesLimit(HkpConeLimitConstraintAtom)),
    ("ballSocket" => BallSocket(HkpBallSocketConstraintAtom)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_TWIST")]
    AxisTwist = 0,
    #[serde(rename = "AXIS_PLANES")]
    AxisPlanes = 1,
    #[serde(rename = "AXIS_CROSS_PRODUCT")]
    AxisCrossProduct = 2,
}
