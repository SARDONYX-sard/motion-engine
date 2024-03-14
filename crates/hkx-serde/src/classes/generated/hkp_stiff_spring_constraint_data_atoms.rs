//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpStiffSpringConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpStiffSpringConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x207eb376`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStiffSpringConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"pivots"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivots")]
    Pivots(HkpSetLocalTranslationsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"spring"`
    /// -   type: `struct hkpStiffSpringConstraintAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spring")]
    Spring(HkpStiffSpringConstraintAtom),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStiffSpringConstraintDataAtoms, "@name",
    ("pivots" => Pivots(HkpSetLocalTranslationsConstraintAtom)),
    ("spring" => Spring(HkpStiffSpringConstraintAtom)),
}
