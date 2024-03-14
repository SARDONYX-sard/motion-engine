//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpLimitedForceConstraintMotor`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpLimitedForceConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkpConstraintMotor`/`0x6a44c317`
/// - signature: `0x3377b0b0`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLimitedForceConstraintMotor {
    /// # C++ Class Fields Info
    /// -   name:`"minForce"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minForce")]
    MinForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLimitedForceConstraintMotor, "@name",
    ("minForce" => MinForce(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
}
