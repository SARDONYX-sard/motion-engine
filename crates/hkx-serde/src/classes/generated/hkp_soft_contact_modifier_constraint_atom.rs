//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSoftContactModifierConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSoftContactModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0xecb34e27`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSoftContactModifierConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAcceleration"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAcceleration")]
    MaxAcceleration(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSoftContactModifierConstraintAtom, "@name",
    ("tau" => Tau(Primitive<f32>)),
    ("maxAcceleration" => MaxAcceleration(Primitive<f32>)),
}
