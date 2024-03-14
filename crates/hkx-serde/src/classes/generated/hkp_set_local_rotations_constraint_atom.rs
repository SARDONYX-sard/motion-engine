//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSetLocalRotationsConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSetLocalRotationsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf81db8e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalRotationsConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"rotationA"`
    /// -   type: `hkRotation`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationA")]
    RotationA(Rotation<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationB"`
    /// -   type: `hkRotation`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationB")]
    RotationB(Rotation<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalRotationsConstraintAtom, "@name",
    ("rotationA" => RotationA(Rotation<f32>)),
    ("rotationB" => RotationB(Rotation<f32>)),
}
