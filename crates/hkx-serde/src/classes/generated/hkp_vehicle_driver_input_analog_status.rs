//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpVehicleDriverInputAnalogStatus`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDriverInputAnalogStatus`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkpVehicleDriverInputStatus`/`0xda8c7d7d`
/// - signature: `0x2b4a5803`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDriverInputAnalogStatus {
    /// # C++ Class Fields Info
    /// -   name:`"positionX"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionX")]
    PositionX(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"positionY"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionY")]
    PositionY(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handbrakeButtonPressed"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handbrakeButtonPressed")]
    HandbrakeButtonPressed(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"reverseButtonPressed"`
    /// -   type: `hkBool`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseButtonPressed")]
    ReverseButtonPressed(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDriverInputAnalogStatus, "@name",
    ("positionX" => PositionX(Primitive<f32>)),
    ("positionY" => PositionY(Primitive<f32>)),
    ("handbrakeButtonPressed" => HandbrakeButtonPressed(Primitive<bool>)),
    ("reverseButtonPressed" => ReverseButtonPressed(Primitive<bool>)),
}
