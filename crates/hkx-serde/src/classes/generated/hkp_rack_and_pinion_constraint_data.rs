//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRackAndPinionConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpRackAndPinionConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0xd180ebe0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRackAndPinionConstraintData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpRackAndPinionConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpRackAndPinionConstraintDataAtoms>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRackAndPinionConstraintData, "@name",
    ("atoms" => Atoms(SingleClass<HkpRackAndPinionConstraintDataAtoms>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TYPE_RACK_AND_PINION")]
    TypeRackAndPinion = 0,
    #[serde(rename = "TYPE_SCREW")]
    TypeScrew = 1,
}
