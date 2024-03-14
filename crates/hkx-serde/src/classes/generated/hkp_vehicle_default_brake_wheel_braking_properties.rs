//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultBrakeWheelBrakingProperties`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultBrakeWheelBrakingProperties`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x1ffad971`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultBrakeWheelBrakingProperties {
    /// # C++ Class Fields Info
    /// -   name:`"maxBreakingTorque"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxBreakingTorque")]
    MaxBreakingTorque(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minPedalInputToBlock"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minPedalInputToBlock")]
    MinPedalInputToBlock(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isConnectedToHandbrake"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isConnectedToHandbrake")]
    IsConnectedToHandbrake(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultBrakeWheelBrakingProperties, "@name",
    ("maxBreakingTorque" => MaxBreakingTorque(Primitive<f32>)),
    ("minPedalInputToBlock" => MinPedalInputToBlock(Primitive<f32>)),
    ("isConnectedToHandbrake" => IsConnectedToHandbrake(Primitive<bool>)),
}
