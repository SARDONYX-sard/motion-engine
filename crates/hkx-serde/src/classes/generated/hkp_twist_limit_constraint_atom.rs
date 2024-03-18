//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTwistLimitConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpTwistLimitConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x7c9b1052`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTwistLimitConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"twistAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistAxis")]
    TwistAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"refAxis"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refAxis")]
    RefAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"minAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAngle")]
    MinAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angularLimitsTauFactor"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularLimitsTauFactor")]
    AngularLimitsTauFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTwistLimitConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("twistAxis" => TwistAxis(Primitive<u8>)),
    ("refAxis" => RefAxis(Primitive<u8>)),
    ("minAngle" => MinAngle(Primitive<f32>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(Primitive<f32>)),
}
