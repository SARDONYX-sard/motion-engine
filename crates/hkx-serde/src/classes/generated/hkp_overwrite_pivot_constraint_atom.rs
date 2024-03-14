//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpOverwritePivotConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpOverwritePivotConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x1f11b467`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpOverwritePivotConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"copyToPivotBFromPivotA"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "copyToPivotBFromPivotA")]
    CopyToPivotBFromPivotA(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpOverwritePivotConstraintAtom, "@name",
    ("copyToPivotBFromPivotA" => CopyToPivotBFromPivotA(Primitive<u8>)),
}
