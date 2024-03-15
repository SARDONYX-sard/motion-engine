//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMalleableConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMalleableConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x6748b2cf`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMalleableConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintData")]
    ConstraintData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMalleableConstraintData<'de>, "@name",
    ("constraintData" => ConstraintData(Primitive<Cow<'de, str>>)),
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("strength" => Strength(Primitive<f32>)),
}
