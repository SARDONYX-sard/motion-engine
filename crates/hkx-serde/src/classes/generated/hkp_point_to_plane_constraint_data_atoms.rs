//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPointToPlaneConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpPointToPlaneConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: false
/// - signature: `0x749bc260`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPointToPlaneConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"lin"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin")]
    Lin(SingleClass<HkpLinConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPointToPlaneConstraintDataAtoms, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("lin" => Lin(SingleClass<HkpLinConstraintAtom>)),
}
