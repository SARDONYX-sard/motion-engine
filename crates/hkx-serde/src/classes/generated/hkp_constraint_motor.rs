//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConstraintMotor`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x6a44c317`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintMotor {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum MotorType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(MotorType),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintMotor, "@name",
    ("type" => Type(MotorType)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MotorType {
    #[serde(rename = "TYPE_INVALID")]
    TypeInvalid = 0,
    #[serde(rename = "TYPE_POSITION")]
    TypePosition = 1,
    #[serde(rename = "TYPE_VELOCITY")]
    TypeVelocity = 2,
    #[serde(rename = "TYPE_SPRING_DAMPER")]
    TypeSpringDamper = 3,
    #[serde(rename = "TYPE_CALLBACK")]
    TypeCallback = 4,
    #[serde(rename = "TYPE_MAX")]
    TypeMax = 5,
}
