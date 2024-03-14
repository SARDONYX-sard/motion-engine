//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpHingeLimitsData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpHingeLimitsData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0xbd46760a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpHingeLimitsData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpHingeLimitsDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpHingeLimitsDataAtoms),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeLimitsData, "@name",
    ("atoms" => Atoms(HkpHingeLimitsDataAtoms)),
}
