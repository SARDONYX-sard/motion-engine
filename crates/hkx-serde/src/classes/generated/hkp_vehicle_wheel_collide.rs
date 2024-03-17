//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleWheelCollide`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x4a50fcb`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleWheelCollide {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"alreadyUsed"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alreadyUsed")]
    AlreadyUsed(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleWheelCollide, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("alreadyUsed" => AlreadyUsed(Primitive<bool>)),
    ("type" => Type(Primitive<()>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WheelCollideType {
    #[serde(rename = "INVALID_WHEEL_COLLIDE")]
    InvalidWheelCollide = 0,
    #[serde(rename = "RAY_CAST_WHEEL_COLLIDE")]
    RayCastWheelCollide = 1,
    #[serde(rename = "LINEAR_CAST_WHEEL_COLLIDE")]
    LinearCastWheelCollide = 2,
    #[serde(rename = "USER_WHEEL_COLLIDE1")]
    UserWheelCollide1 = 3,
    #[serde(rename = "USER_WHEEL_COLLIDE2")]
    UserWheelCollide2 = 4,
    #[serde(rename = "USER_WHEEL_COLLIDE3")]
    UserWheelCollide3 = 5,
    #[serde(rename = "USER_WHEEL_COLLIDE4")]
    UserWheelCollide4 = 6,
    #[serde(rename = "USER_WHEEL_COLLIDE5")]
    UserWheelCollide5 = 7,
}
