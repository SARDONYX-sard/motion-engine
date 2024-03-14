//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpMovingSurfaceModifierConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMovingSurfaceModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0x79ab517d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMovingSurfaceModifierConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"velocity"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocity")]
    Velocity(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMovingSurfaceModifierConstraintAtom, "@name",
    ("velocity" => Velocity(Vector4<f32>)),
}
