//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleRayCastWheelCollide`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleRayCastWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpVehicleWheelCollide`/`0x4a50fcb`
/// - signature: `0x41efd9e3`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleRayCastWheelCollide<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"wheelCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollisionFilterInfo")]
    WheelCollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantom")]
    Phantom(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rejectRayChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rejectRayChassisListener")]
    RejectRayChassisListener(HkpRejectChassisListener),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleRayCastWheelCollide<'de>, "@name",
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(Primitive<u32>)),
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("rejectRayChassisListener" => RejectRayChassisListener(HkpRejectChassisListener)),
}
