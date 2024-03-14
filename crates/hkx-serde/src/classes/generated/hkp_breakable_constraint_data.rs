//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBreakableConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBreakableConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x7d6310c8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBreakableConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintData")]
    ConstraintData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"childRuntimeSize"`
    /// -   type: `hkUint16`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childRuntimeSize")]
    ChildRuntimeSize(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"childNumSolverResults"`
    /// -   type: `hkUint16`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childNumSolverResults")]
    ChildNumSolverResults(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"solverResultLimit"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverResultLimit")]
    SolverResultLimit(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"removeWhenBroken"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "removeWhenBroken")]
    RemoveWhenBroken(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"revertBackVelocityOnBreak"`
    /// -   type: `hkBool`
    /// - offset: 37
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "revertBackVelocityOnBreak")]
    RevertBackVelocityOnBreak(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBreakableConstraintData<'de>, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("constraintData" => ConstraintData(Cow<'de, str>)),
    ("childRuntimeSize" => ChildRuntimeSize(Primitive<u16>)),
    ("childNumSolverResults" => ChildNumSolverResults(Primitive<u16>)),
    ("solverResultLimit" => SolverResultLimit(Primitive<f32>)),
    ("removeWhenBroken" => RemoveWhenBroken(Primitive<bool>)),
    ("revertBackVelocityOnBreak" => RevertBackVelocityOnBreak(Primitive<bool>)),
}
