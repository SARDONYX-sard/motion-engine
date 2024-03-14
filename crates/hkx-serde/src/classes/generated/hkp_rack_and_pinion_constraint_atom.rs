//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpRackAndPinionConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpRackAndPinionConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x30cae006`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRackAndPinionConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"pinionRadiusOrScrewPitch"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pinionRadiusOrScrewPitch")]
    PinionRadiusOrScrewPitch(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isScrew"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isScrew")]
    IsScrew(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToInitialAngleOffset"`
    /// -   type: `hkInt8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToInitialAngleOffset")]
    MemOffsetToInitialAngleOffset(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToPrevAngle"`
    /// -   type: `hkInt8`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToPrevAngle")]
    MemOffsetToPrevAngle(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToRevolutionCounter"`
    /// -   type: `hkInt8`
    /// - offset: 11
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToRevolutionCounter")]
    MemOffsetToRevolutionCounter(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRackAndPinionConstraintAtom, "@name",
    ("pinionRadiusOrScrewPitch" => PinionRadiusOrScrewPitch(Primitive<f32>)),
    ("isScrew" => IsScrew(Primitive<bool>)),
    ("memOffsetToInitialAngleOffset" => MemOffsetToInitialAngleOffset(Primitive<i8>)),
    ("memOffsetToPrevAngle" => MemOffsetToPrevAngle(Primitive<i8>)),
    ("memOffsetToRevolutionCounter" => MemOffsetToRevolutionCounter(Primitive<i8>)),
}
