//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAngMotorConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpAngMotorConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x81f087ff`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngMotorConstraintAtom<'a> {
    /// # C++ Parent class(`hkpConstraintAtom`, parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled", default)]
    IsEnabled(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"motorAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motorAxis", default)]
    MotorAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializedOffset", default)]
    InitializedOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"previousTargetAngleOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousTargetAngleOffset", default)]
    PreviousTargetAngleOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"correspondingAngLimitSolverResultOffset"`
    /// -   type: `hkInt16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "correspondingAngLimitSolverResultOffset", default)]
    CorrespondingAngLimitSolverResultOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"targetAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetAngle", default)]
    TargetAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motor", default)]
    Motor(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngMotorConstraintAtom<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
    ("motorAxis" => MotorAxis(Primitive<u8>)),
    ("initializedOffset" => InitializedOffset(Primitive<i16>)),
    ("previousTargetAngleOffset" => PreviousTargetAngleOffset(Primitive<i16>)),
    ("correspondingAngLimitSolverResultOffset" => CorrespondingAngLimitSolverResultOffset(Primitive<i16>)),
    ("targetAngle" => TargetAngle(Primitive<f32>)),
    ("motor" => Motor(Primitive<Cow<'de, str>>)),
}
