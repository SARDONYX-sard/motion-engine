//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMassChangerModifierConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMassChangerModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0xb6b28240`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMassChangerModifierConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"factorA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "factorA")]
    FactorA(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"factorB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "factorB")]
    FactorB(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMassChangerModifierConstraintAtom, "@name",
    ("factorA" => FactorA(Vector4<f32>)),
    ("factorB" => FactorB(Vector4<f32>)),
}
