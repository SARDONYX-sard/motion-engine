//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinMotorConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpLinMotorConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x10312464`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinMotorConstraintAtom<'a> {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"motorAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motorAxis")]
    MotorAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializedOffset")]
    InitializedOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"previousTargetPositionOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousTargetPositionOffset")]
    PreviousTargetPositionOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motor")]
    Motor(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinMotorConstraintAtom<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
    ("motorAxis" => MotorAxis(Primitive<u8>)),
    ("initializedOffset" => InitializedOffset(Primitive<i16>)),
    ("previousTargetPositionOffset" => PreviousTargetPositionOffset(Primitive<i16>)),
    ("targetPosition" => TargetPosition(Primitive<f32>)),
    ("motor" => Motor(Primitive<Cow<'de, str>>)),
}
