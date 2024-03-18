//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallSocketConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpBallSocketConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xe70e4dfa`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallSocketConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"solvingMethod"`
    /// -   type: `enum SolvingMethod`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solvingMethod")]
    SolvingMethod(Primitive<SolvingMethod>),
    /// # C++ Class Fields Info
    /// -   name:`"bodiesToNotify"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodiesToNotify")]
    BodiesToNotify(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"velocityStabilizationFactor"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityStabilizationFactor")]
    VelocityStabilizationFactor(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"maxImpulse"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxImpulse")]
    MaxImpulse(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"inertiaStabilizationFactor"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaStabilizationFactor")]
    InertiaStabilizationFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallSocketConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("solvingMethod" => SolvingMethod(Primitive<SolvingMethod>)),
    ("bodiesToNotify" => BodiesToNotify(Primitive<u8>)),
    ("velocityStabilizationFactor" => VelocityStabilizationFactor(Primitive<u8>)),
    ("maxImpulse" => MaxImpulse(Primitive<f32>)),
    ("inertiaStabilizationFactor" => InertiaStabilizationFactor(Primitive<f32>)),
}
