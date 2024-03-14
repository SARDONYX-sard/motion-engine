//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpMotorAction`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMotorAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x8ff131d9`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMotorAction {
    /// # C++ Class Fields Info
    /// -   name:`"axis"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axis")]
    Axis(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"spinRate"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinRate")]
    SpinRate(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "active")]
    Active(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMotorAction, "@name",
    ("axis" => Axis(Vector4<f32>)),
    ("spinRate" => SpinRate(Primitive<f32>)),
    ("gain" => Gain(Primitive<f32>)),
    ("active" => Active(Primitive<bool>)),
}
