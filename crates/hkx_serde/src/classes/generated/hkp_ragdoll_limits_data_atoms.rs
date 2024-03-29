//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollLimitsDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpRagdollLimitsDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: false
/// - signature: `0x82b894c3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollLimitsDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotations")]
    Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistLimit")]
    TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "coneLimit")]
    ConeLimit(SingleClass<HkpConeLimitConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planesLimit")]
    PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollLimitsDataAtoms, "@name",
    ("rotations" => Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>)),
    ("twistLimit" => TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>)),
    ("coneLimit" => ConeLimit(SingleClass<HkpConeLimitConstraintAtom>)),
    ("planesLimit" => PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>)),
}

impl ByteDeSerialize for HkpRagdollLimitsDataAtoms {
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
