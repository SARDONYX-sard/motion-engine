//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDataWheelComponentParams`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleDataWheelComponentParams`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: false
/// - signature: `0x82fe40e0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDataWheelComponentParams {
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"width"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "width")]
    Width(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"viscosityFriction"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "viscosityFriction")]
    ViscosityFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxFriction"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFriction")]
    MaxFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"slipAngle"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "slipAngle")]
    SlipAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"forceFeedbackMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceFeedbackMultiplier")]
    ForceFeedbackMultiplier(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxContactBodyAcceleration"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxContactBodyAcceleration")]
    MaxContactBodyAcceleration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"axle"`
    /// -   type: `hkInt8`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axle")]
    Axle(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDataWheelComponentParams, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("width" => Width(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("viscosityFriction" => ViscosityFriction(Primitive<f32>)),
    ("maxFriction" => MaxFriction(Primitive<f32>)),
    ("slipAngle" => SlipAngle(Primitive<f32>)),
    ("forceFeedbackMultiplier" => ForceFeedbackMultiplier(Primitive<f32>)),
    ("maxContactBodyAcceleration" => MaxContactBodyAcceleration(Primitive<f32>)),
    ("axle" => Axle(Primitive<i8>)),
}

impl ByteDeSerialize for HkpVehicleDataWheelComponentParams {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
