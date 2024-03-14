//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpRackAndPinionConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpRackAndPinionConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xa58a9659`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRackAndPinionConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkpSetLocalTransformsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"rackAndPinion"`
    /// -   type: `struct hkpRackAndPinionConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rackAndPinion")]
    RackAndPinion(HkpRackAndPinionConstraintAtom),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRackAndPinionConstraintDataAtoms, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("rackAndPinion" => RackAndPinion(HkpRackAndPinionConstraintAtom)),
}
