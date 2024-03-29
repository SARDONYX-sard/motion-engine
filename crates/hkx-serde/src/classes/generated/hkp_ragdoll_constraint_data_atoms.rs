//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollConstraintDataAtoms<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setupStabilization")]
    SetupStabilization(SingleClass<HkpSetupStabilizationAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollMotors"`
    /// -   type: `struct hkpRagdollMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollMotors")]
    RagdollMotors(SingleClass<HkpRagdollMotorConstraintAtom<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angFriction")]
    AngFriction(SingleClass<HkpAngFrictionConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistLimit")]
    TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "coneLimit")]
    ConeLimit(SingleClass<HkpConeLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 292
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planesLimit")]
    PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollConstraintDataAtoms<'de>, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("ragdollMotors" => RagdollMotors(SingleClass<HkpRagdollMotorConstraintAtom<'de>>)),
    ("angFriction" => AngFriction(SingleClass<HkpAngFrictionConstraintAtom>)),
    ("twistLimit" => TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>)),
    ("coneLimit" => ConeLimit(SingleClass<HkpConeLimitConstraintAtom>)),
    ("planesLimit" => PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

impl ByteDeSerialize for HkpRagdollConstraintDataAtoms<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Axis {
    #[serde(rename = "AXIS_TWIST")]
    AxisTwist = 0,
    #[serde(rename = "AXIS_PLANES")]
    AxisPlanes = 1,
    #[serde(rename = "AXIS_CROSS_PRODUCT")]
    AxisCrossProduct = 2,
}
