//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPrismaticConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpPrismaticConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 192
/// -    vtable: false
/// - signature: `0x7f516137`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPrismaticConstraintDataAtoms<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpLinMotorConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motor")]
    Motor(SingleClass<HkpLinMotorConstraintAtom<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `struct hkpLinFrictionConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(SingleClass<HkpLinFrictionConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ang"`
    /// -   type: `struct hkpAngConstraintAtom`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ang")]
    Ang(SingleClass<HkpAngConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"lin0"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0")]
    Lin0(SingleClass<HkpLinConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"lin1"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin1")]
    Lin1(SingleClass<HkpLinConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"linLimit"`
    /// -   type: `struct hkpLinLimitConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linLimit")]
    LinLimit(SingleClass<HkpLinLimitConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPrismaticConstraintDataAtoms<'de>, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("motor" => Motor(SingleClass<HkpLinMotorConstraintAtom<'de>>)),
    ("friction" => Friction(SingleClass<HkpLinFrictionConstraintAtom>)),
    ("ang" => Ang(SingleClass<HkpAngConstraintAtom>)),
    ("lin0" => Lin0(SingleClass<HkpLinConstraintAtom>)),
    ("lin1" => Lin1(SingleClass<HkpLinConstraintAtom>)),
    ("linLimit" => LinLimit(SingleClass<HkpLinLimitConstraintAtom>)),
}

impl ByteDeSerialize for HkpPrismaticConstraintDataAtoms<'_> {
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
    #[serde(rename = "AXIS_SHAFT")]
    AxisShaft = 0,
    #[serde(rename = "AXIS_PERP_TO_SHAFT")]
    AxisPerpToShaft = 1,
}
