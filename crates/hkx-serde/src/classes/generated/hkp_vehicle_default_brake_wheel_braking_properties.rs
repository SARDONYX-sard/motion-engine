//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultBrakeWheelBrakingProperties`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleDefaultBrakeWheelBrakingProperties`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x1ffad971`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
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

impl ByteDeSerialize for HkpVehicleDefaultBrakeWheelBrakingProperties {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
