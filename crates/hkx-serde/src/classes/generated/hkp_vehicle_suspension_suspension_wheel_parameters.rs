//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleSuspensionSuspensionWheelParameters`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleSuspensionSuspensionWheelParameters`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x358bfe9c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleSuspensionSuspensionWheelParameters {
    /// # C++ Class Fields Info
    /// -   name:`"hardpointChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hardpointChassisSpace")]
    HardpointChassisSpace(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"directionChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "directionChassisSpace")]
    DirectionChassisSpace(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"length"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "length")]
    Length(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleSuspensionSuspensionWheelParameters, "@name",
    ("hardpointChassisSpace" => HardpointChassisSpace(Primitive<Vector4<f32>>)),
    ("directionChassisSpace" => DirectionChassisSpace(Primitive<Vector4<f32>>)),
    ("length" => Length(Primitive<f32>)),
}
