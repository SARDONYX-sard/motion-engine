//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpAngConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpAngConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x35bb3cd0`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"firstConstrainedAxis"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "firstConstrainedAxis")]
    FirstConstrainedAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"numConstrainedAxes"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numConstrainedAxes")]
    NumConstrainedAxes(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngConstraintAtom, "@name",
    ("firstConstrainedAxis" => FirstConstrainedAxis(Primitive<u8>)),
    ("numConstrainedAxes" => NumConstrainedAxes(Primitive<u8>)),
}
