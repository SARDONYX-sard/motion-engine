//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpPositionConstraintMotor`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPositionConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0x748fb303`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPositionConstraintMotor {
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"proportionalRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "proportionalRecoveryVelocity")]
    ProportionalRecoveryVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"constantRecoveryVelocity"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constantRecoveryVelocity")]
    ConstantRecoveryVelocity(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPositionConstraintMotor, "@name",
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("proportionalRecoveryVelocity" => ProportionalRecoveryVelocity(Primitive<f32>)),
    ("constantRecoveryVelocity" => ConstantRecoveryVelocity(Primitive<f32>)),
}
