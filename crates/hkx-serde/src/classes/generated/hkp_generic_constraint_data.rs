//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGenericConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"scheme"`
    /// -   type: `struct hkpGenericConstraintDataScheme`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scheme")]
    Scheme(SingleClass<HkpGenericConstraintDataScheme<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintData<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("scheme" => Scheme(SingleClass<HkpGenericConstraintDataScheme<'de>>)),
}
