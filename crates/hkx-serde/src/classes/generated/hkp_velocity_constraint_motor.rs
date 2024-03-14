//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVelocityConstraintMotor`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVelocityConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0xfca2fcc3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVelocityConstraintMotor {
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"velocityTarget"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityTarget")]
    VelocityTarget(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"useVelocityTargetFromConstraintTargets"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useVelocityTargetFromConstraintTargets")]
    UseVelocityTargetFromConstraintTargets(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVelocityConstraintMotor, "@name",
    ("tau" => Tau(Primitive<f32>)),
    ("velocityTarget" => VelocityTarget(Primitive<f32>)),
    ("useVelocityTargetFromConstraintTargets" => UseVelocityTargetFromConstraintTargets(Primitive<bool>)),
}
