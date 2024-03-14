//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpVehicleDefaultSteering`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultSteering`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkpVehicleSteering`/`0xda8c7d7d`
/// - signature: `0x8f0411c8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultSteering {
    /// # C++ Class Fields Info
    /// -   name:`"maxSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSteeringAngle")]
    MaxSteeringAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSpeedFullSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSpeedFullSteeringAngle")]
    MaxSpeedFullSteeringAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"doesWheelSteer"`
    /// -   type: `hkArray&lt;hkBool&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "doesWheelSteer")]
    DoesWheelSteer(HkArrayRef<Primitive<bool>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSteering, "@name",
    ("maxSteeringAngle" => MaxSteeringAngle(Primitive<f32>)),
    ("maxSpeedFullSteeringAngle" => MaxSpeedFullSteeringAngle(Primitive<f32>)),
    ("doesWheelSteer" => DoesWheelSteer(HkArrayRef<Primitive<bool>>)),
}
