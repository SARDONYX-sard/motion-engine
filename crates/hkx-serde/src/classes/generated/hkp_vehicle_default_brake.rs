//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpVehicleDefaultBrake`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultBrake`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkpVehicleBrake`/`0xda8c7d7d`
/// - signature: `0x4b4f8816`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultBrake {
    /// # C++ Class Fields Info
    /// -   name:`"wheelBrakingProperties"`
    /// -   type: `hkArray&lt;struct hkpVehicleDefaultBrakeWheelBrakingProperties&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelBrakingProperties")]
    WheelBrakingProperties(HkArrayClass<HkpVehicleDefaultBrakeWheelBrakingProperties>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelsMinTimeToBlock"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsMinTimeToBlock")]
    WheelsMinTimeToBlock(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultBrake, "@name",
    ("wheelBrakingProperties" => WheelBrakingProperties(HkArrayClass<HkpVehicleDefaultBrakeWheelBrakingProperties>)),
    ("wheelsMinTimeToBlock" => WheelsMinTimeToBlock(Primitive<f32>)),
}
