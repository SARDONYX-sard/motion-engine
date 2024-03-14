//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpViscousSurfaceModifierConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpViscousSurfaceModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0x5c6aa14d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpViscousSurfaceModifierConstraintAtom {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpViscousSurfaceModifierConstraintAtom, "@name",
}
