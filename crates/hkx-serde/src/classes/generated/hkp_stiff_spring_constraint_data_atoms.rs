//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStiffSpringConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpStiffSpringConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x207eb376`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStiffSpringConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"pivots"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivots")]
    Pivots(SingleClass<HkpSetLocalTranslationsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"spring"`
    /// -   type: `struct hkpStiffSpringConstraintAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spring")]
    Spring(SingleClass<HkpStiffSpringConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStiffSpringConstraintDataAtoms, "@name",
    ("pivots" => Pivots(SingleClass<HkpSetLocalTranslationsConstraintAtom>)),
    ("spring" => Spring(SingleClass<HkpStiffSpringConstraintAtom>)),
}
