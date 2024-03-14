//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleLinearCastWheelCollide`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleLinearCastWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkpVehicleWheelCollide`/`0x4a50fcb`
/// - signature: `0xc59399d0`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleLinearCastWheelCollide {
    /// # C++ Class Fields Info
    /// -   name:`"wheelCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollisionFilterInfo")]
    WheelCollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelStates"`
    /// -   type: `hkArray&lt;struct hkpVehicleLinearCastWheelCollideWheelState&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelStates")]
    WheelStates(HkArrayClass<HkpVehicleLinearCastWheelCollideWheelState>),
    /// # C++ Class Fields Info
    /// -   name:`"rejectChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rejectChassisListener")]
    RejectChassisListener(HkpRejectChassisListener),
    /// # C++ Class Fields Info
    /// -   name:`"maxExtraPenetration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxExtraPenetration")]
    MaxExtraPenetration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startPointTolerance"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startPointTolerance")]
    StartPointTolerance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollide, "@name",
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(Primitive<u32>)),
    ("wheelStates" => WheelStates(HkArrayClass<HkpVehicleLinearCastWheelCollideWheelState>)),
    ("rejectChassisListener" => RejectChassisListener(HkpRejectChassisListener)),
    ("maxExtraPenetration" => MaxExtraPenetration(Primitive<f32>)),
    ("startPointTolerance" => StartPointTolerance(Primitive<f32>)),
}
