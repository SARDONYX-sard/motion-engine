//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGenericConstraintDataSchemeConstraintInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpGenericConstraintDataSchemeConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xd6421f19`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintDataSchemeConstraintInfo {
    /// # C++ Class Fields Info
    /// -   name:`"maxSizeOfSchema"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSizeOfSchema")]
    MaxSizeOfSchema(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"sizeOfSchemas"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfSchemas")]
    SizeOfSchemas(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numSolverResults"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSolverResults")]
    NumSolverResults(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numSolverElemTemps"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSolverElemTemps")]
    NumSolverElemTemps(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataSchemeConstraintInfo, "@name",
    ("maxSizeOfSchema" => MaxSizeOfSchema(Primitive<i32>)),
    ("sizeOfSchemas" => SizeOfSchemas(Primitive<i32>)),
    ("numSolverResults" => NumSolverResults(Primitive<i32>)),
    ("numSolverElemTemps" => NumSolverElemTemps(Primitive<i32>)),
}
