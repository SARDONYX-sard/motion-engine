//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpHingeLimitsDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpHingeLimitsDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: false
/// - signature: `0x555876ff`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpHingeLimitsDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotations")]
    Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"angLimit"`
    /// -   type: `struct hkpAngLimitConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angLimit")]
    AngLimit(SingleClass<HkpAngLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    _2DAng(SingleClass<Hkp2DAngConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeLimitsDataAtoms, "@name",
    ("rotations" => Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>)),
    ("angLimit" => AngLimit(SingleClass<HkpAngLimitConstraintAtom>)),
    ("2dAng" => _2DAng(SingleClass<Hkp2DAngConstraintAtom>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
    #[serde(rename = "AXIS_PERP_TO_AXLE_1")]
    AxisPerpToAxle1 = 1,
    #[serde(rename = "AXIS_PERP_TO_AXLE_2")]
    AxisPerpToAxle2 = 2,
}
