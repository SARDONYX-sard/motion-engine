//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpPrismaticConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPrismaticConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 192
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x7f516137`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPrismaticConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkpSetLocalTransformsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpLinMotorConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motor")]
    Motor(HkpLinMotorConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `struct hkpLinFrictionConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(HkpLinFrictionConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"ang"`
    /// -   type: `struct hkpAngConstraintAtom`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ang")]
    Ang(HkpAngConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"lin0"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0")]
    Lin0(HkpLinConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"lin1"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin1")]
    Lin1(HkpLinConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"linLimit"`
    /// -   type: `struct hkpLinLimitConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linLimit")]
    LinLimit(HkpLinLimitConstraintAtom),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPrismaticConstraintDataAtoms, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("motor" => Motor(HkpLinMotorConstraintAtom)),
    ("friction" => Friction(HkpLinFrictionConstraintAtom)),
    ("ang" => Ang(HkpAngConstraintAtom)),
    ("lin0" => Lin0(HkpLinConstraintAtom)),
    ("lin1" => Lin1(HkpLinConstraintAtom)),
    ("linLimit" => LinLimit(HkpLinLimitConstraintAtom)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_SHAFT")]
    AxisShaft = 0,
    #[serde(rename = "AXIS_PERP_TO_SHAFT")]
    AxisPerpToShaft = 1,
}
