//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLimitedHingeConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpLimitedHingeConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 240
/// -    vtable: false
/// - signature: `0x54c7715b`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLimitedHingeConstraintDataAtoms<'a> {
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
    /// -   name:`"angMotor"`
    /// -   type: `struct hkpAngMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angMotor")]
    AngMotor(SingleClass<HkpAngMotorConstraintAtom<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angFriction")]
    AngFriction(SingleClass<HkpAngFrictionConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"angLimit"`
    /// -   type: `struct hkpAngLimitConstraintAtom`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angLimit")]
    AngLimit(SingleClass<HkpAngLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    _2DAng(SingleClass<Hkp2DAngConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLimitedHingeConstraintDataAtoms<'de>, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("angMotor" => AngMotor(SingleClass<HkpAngMotorConstraintAtom<'de>>)),
    ("angFriction" => AngFriction(SingleClass<HkpAngFrictionConstraintAtom>)),
    ("angLimit" => AngLimit(SingleClass<HkpAngLimitConstraintAtom>)),
    ("2dAng" => _2DAng(SingleClass<Hkp2DAngConstraintAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

impl ByteDeSerialize for HkpLimitedHingeConstraintDataAtoms<'_> {
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
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
    #[serde(rename = "AXIS_PERP_TO_AXLE_1")]
    AxisPerpToAxle1 = 1,
    #[serde(rename = "AXIS_PERP_TO_AXLE_2")]
    AxisPerpToAxle2 = 2,
}
