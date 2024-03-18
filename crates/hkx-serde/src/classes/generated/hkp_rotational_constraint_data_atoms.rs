//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRotationalConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpRotationalConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: false
/// - signature: `0xa0c64586`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRotationalConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotations")]
    Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ang"`
    /// -   type: `struct hkpAngConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ang")]
    Ang(SingleClass<HkpAngConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRotationalConstraintDataAtoms, "@name",
    ("rotations" => Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>)),
    ("ang" => Ang(SingleClass<HkpAngConstraintAtom>)),
}
