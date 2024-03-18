//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpHingeConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpHingeConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 192
/// -    vtable: false
/// - signature: `0x6958371c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpHingeConstraintDataAtoms {
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
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    _2DAng(SingleClass<Hkp2DAngConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeConstraintDataAtoms, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("2dAng" => _2DAng(SingleClass<Hkp2DAngConstraintAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
}
