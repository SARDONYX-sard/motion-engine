//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCallbackConstraintMotor`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCallbackConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0xafcd79ad`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCallbackConstraintMotor<'a> {
    /// # C++ Parent class(`hkpLimitedForceConstraintMotor`, parent: `hkpConstraintMotor`) field Info
    /// -   name:`"minForce"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minForce", default)]
    MinForce(Primitive<f32>),
    /// # C++ Parent class(`hkpLimitedForceConstraintMotor`, parent: `hkpConstraintMotor`) field Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxForce", default)]
    MaxForce(Primitive<f32>),

    /// # C++ Parent class(`hkpConstraintMotor`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum MotorType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<MotorType>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"callbackFunc"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "callbackFunc", default, skip_serializing)]
    CallbackFunc(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"callbackType"`
    /// -   type: `enum CallbackType`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "callbackType", default)]
    CallbackType(Primitive<CallbackType>),
    /// # C++ Class Fields Info
    /// -   name:`"userData0"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData0", default)]
    UserData0(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"userData1"`
    /// -   type: `hkUlong`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData1", default)]
    UserData1(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"userData2"`
    /// -   type: `hkUlong`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData2", default)]
    UserData2(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCallbackConstraintMotor<'de>, "@name",
    ("minForce" => MinForce(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("type" => Type(Primitive<MotorType>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("callbackFunc" => CallbackFunc(Primitive<Cow<'de, str>>)),
    ("callbackType" => CallbackType(Primitive<CallbackType>)),
    ("userData0" => UserData0(Primitive<usize>)),
    ("userData1" => UserData1(Primitive<usize>)),
    ("userData2" => UserData2(Primitive<usize>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CallbackType {
    #[serde(rename = "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER")]
    CallbackMotorTypeHavokDemoSpringDamper = 0,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_0")]
    CallbackMotorTypeUser0 = 1,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_1")]
    CallbackMotorTypeUser1 = 2,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_2")]
    CallbackMotorTypeUser2 = 3,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_3")]
    CallbackMotorTypeUser3 = 4,
}
