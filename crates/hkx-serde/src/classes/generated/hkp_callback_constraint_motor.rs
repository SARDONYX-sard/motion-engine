//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpCallbackConstraintMotor`
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
    /// # C++ Class Fields Info
    /// -   name:`"callbackFunc"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "callbackFunc", skip_serializing)]
    CallbackFunc(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"callbackType"`
    /// -   type: `enum CallbackType`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "callbackType")]
    CallbackType(CallbackType),
    /// # C++ Class Fields Info
    /// -   name:`"userData0"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData0")]
    UserData0(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"userData1"`
    /// -   type: `hkUlong`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData1")]
    UserData1(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"userData2"`
    /// -   type: `hkUlong`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData2")]
    UserData2(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCallbackConstraintMotor<'de>, "@name",
    ("callbackFunc" => CallbackFunc(Cow<'de, str>)),
    ("callbackType" => CallbackType(CallbackType)),
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
