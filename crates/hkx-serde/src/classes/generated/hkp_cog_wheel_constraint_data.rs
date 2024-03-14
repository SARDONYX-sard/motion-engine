//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpCogWheelConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCogWheelConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x7f0e53fc`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCogWheelConstraintData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpCogWheelConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpCogWheelConstraintDataAtoms),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCogWheelConstraintData, "@name",
    ("atoms" => Atoms(HkpCogWheelConstraintDataAtoms)),
}
