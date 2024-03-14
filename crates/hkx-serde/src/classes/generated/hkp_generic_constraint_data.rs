//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpGenericConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpGenericConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0xfa824640`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # C++ Class Fields Info
    /// -   name:`"scheme"`
    /// -   type: `struct hkpGenericConstraintDataScheme`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scheme")]
    Scheme(HkpGenericConstraintDataScheme),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintData, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("scheme" => Scheme(HkpGenericConstraintDataScheme)),
}
