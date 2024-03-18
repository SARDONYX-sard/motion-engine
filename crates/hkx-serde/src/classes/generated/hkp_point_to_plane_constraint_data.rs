//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPointToPlaneConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpPointToPlaneConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x65c56e17`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPointToPlaneConstraintData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpPointToPlaneConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpPointToPlaneConstraintDataAtoms>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPointToPlaneConstraintData, "@name",
    ("atoms" => Atoms(SingleClass<HkpPointToPlaneConstraintDataAtoms>)),
}
