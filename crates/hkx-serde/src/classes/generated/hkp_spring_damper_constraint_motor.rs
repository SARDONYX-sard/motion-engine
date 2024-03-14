//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSpringDamperConstraintMotor`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSpringDamperConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0x7ead26f6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSpringDamperConstraintMotor {
    /// # C++ Class Fields Info
    /// -   name:`"springConstant"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springConstant")]
    SpringConstant(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"springDamping"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springDamping")]
    SpringDamping(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSpringDamperConstraintMotor, "@name",
    ("springConstant" => SpringConstant(Primitive<f32>)),
    ("springDamping" => SpringDamping(Primitive<f32>)),
}
