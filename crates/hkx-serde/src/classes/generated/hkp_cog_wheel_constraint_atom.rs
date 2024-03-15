//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCogWheelConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCogWheelConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf2b1f399`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCogWheelConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom`, parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"cogWheelRadiusA"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cogWheelRadiusA", default)]
    CogWheelRadiusA(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cogWheelRadiusB"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cogWheelRadiusB", default)]
    CogWheelRadiusB(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isScrew"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isScrew", default)]
    IsScrew(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToInitialAngleOffset"`
    /// -   type: `hkInt8`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToInitialAngleOffset", default)]
    MemOffsetToInitialAngleOffset(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToPrevAngle"`
    /// -   type: `hkInt8`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToPrevAngle", default)]
    MemOffsetToPrevAngle(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToRevolutionCounter"`
    /// -   type: `hkInt8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToRevolutionCounter", default)]
    MemOffsetToRevolutionCounter(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCogWheelConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("cogWheelRadiusA" => CogWheelRadiusA(Primitive<f32>)),
    ("cogWheelRadiusB" => CogWheelRadiusB(Primitive<f32>)),
    ("isScrew" => IsScrew(Primitive<bool>)),
    ("memOffsetToInitialAngleOffset" => MemOffsetToInitialAngleOffset(Primitive<i8>)),
    ("memOffsetToPrevAngle" => MemOffsetToPrevAngle(Primitive<i8>)),
    ("memOffsetToRevolutionCounter" => MemOffsetToRevolutionCounter(Primitive<i8>)),
}
