//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpRagdollMotorConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpRagdollMotorConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x71013826`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollMotorConstraintAtom<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"initializedOffset"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializedOffset")]
    InitializedOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"previousTargetAnglesOffset"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousTargetAnglesOffset")]
    PreviousTargetAnglesOffset(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"target_bRca"`
    /// -   type: `hkMatrix3`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "target_bRca")]
    TargetBRca(Matrix3<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollMotorConstraintAtom<'de>, "@name",
    ("isEnabled" => IsEnabled(Primitive<bool>)),
    ("initializedOffset" => InitializedOffset(Primitive<i16>)),
    ("previousTargetAnglesOffset" => PreviousTargetAnglesOffset(Primitive<i16>)),
    ("target_bRca" => TargetBRca(Matrix3<f32>)),
    ("motors" => Motors(Cow<'de, str>)),
}
