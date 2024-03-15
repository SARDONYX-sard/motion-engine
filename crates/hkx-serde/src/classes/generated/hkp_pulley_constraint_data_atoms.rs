//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPulleyConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPulleyConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0xb149e5a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPulleyConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"translations"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translations", default)]
    Translations(HkpSetLocalTranslationsConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"pulley"`
    /// -   type: `struct hkpPulleyConstraintAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pulley", default)]
    Pulley(HkpPulleyConstraintAtom),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintDataAtoms, "@name",
    ("translations" => Translations(HkpSetLocalTranslationsConstraintAtom)),
    ("pulley" => Pulley(HkpPulleyConstraintAtom)),
}
