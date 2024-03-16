//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConeLimitConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConeLimitConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf19443c8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConeLimitConstraintAtom {
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
    /// -   name:`"twistAxisInA"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistAxisInA")]
    TwistAxisInA(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"refAxisInB"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refAxisInB")]
    RefAxisInB(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"angleMeasurementMode"`
    /// -   type: `enum MeasurementMode`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angleMeasurementMode")]
    AngleMeasurementMode(Primitive<MeasurementMode>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToAngleOffset"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToAngleOffset")]
    MemOffsetToAngleOffset(Primitive<u8>),
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
    HkpConeLimitConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("twistAxisInA" => TwistAxisInA(Primitive<u8>)),
    ("refAxisInB" => RefAxisInB(Primitive<u8>)),
    ("angleMeasurementMode" => AngleMeasurementMode(Primitive<MeasurementMode>)),
    ("memOffsetToAngleOffset" => MemOffsetToAngleOffset(Primitive<u8>)),
    ("minAngle" => MinAngle(Primitive<f32>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeasurementMode {
    #[serde(rename = "ZERO_WHEN_VECTORS_ALIGNED")]
    ZeroWhenVectorsAligned = 0,
    #[serde(rename = "ZERO_WHEN_VECTORS_PERPENDICULAR")]
    ZeroWhenVectorsPerpendicular = 1,
}
