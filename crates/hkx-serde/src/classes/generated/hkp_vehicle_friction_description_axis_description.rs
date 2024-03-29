//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionDescriptionAxisDescription`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleFrictionDescriptionAxisDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 100
/// -    vtable: false
/// - signature: `0x59ce153f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionDescriptionAxisDescription {
    /// # C++ Class Fields Info
    /// -   name:`"frictionCircleYtab"`
    /// -   type: `hkReal[16]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionCircleYtab")]
    FrictionCircleYtab(CStyleArray<[f32; 16]>),
    /// # C++ Class Fields Info
    /// -   name:`"xStep"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xStep")]
    XStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"xStart"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xStart")]
    XStart(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelSurfaceInertia"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelSurfaceInertia")]
    WheelSurfaceInertia(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelSurfaceInertiaInv"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelSurfaceInertiaInv")]
    WheelSurfaceInertiaInv(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelChassisMassRatio"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelChassisMassRatio")]
    WheelChassisMassRatio(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelRadius"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelRadius")]
    WheelRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelRadiusInv"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelRadiusInv")]
    WheelRadiusInv(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelDownForceFactor"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDownForceFactor")]
    WheelDownForceFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelDownForceSumFactor"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDownForceSumFactor")]
    WheelDownForceSumFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescriptionAxisDescription, "@name",
    ("frictionCircleYtab" => FrictionCircleYtab(CStyleArray<[f32; 16]>)),
    ("xStep" => XStep(Primitive<f32>)),
    ("xStart" => XStart(Primitive<f32>)),
    ("wheelSurfaceInertia" => WheelSurfaceInertia(Primitive<f32>)),
    ("wheelSurfaceInertiaInv" => WheelSurfaceInertiaInv(Primitive<f32>)),
    ("wheelChassisMassRatio" => WheelChassisMassRatio(Primitive<f32>)),
    ("wheelRadius" => WheelRadius(Primitive<f32>)),
    ("wheelRadiusInv" => WheelRadiusInv(Primitive<f32>)),
    ("wheelDownForceFactor" => WheelDownForceFactor(Primitive<f32>)),
    ("wheelDownForceSumFactor" => WheelDownForceSumFactor(Primitive<f32>)),
}

impl ByteDeSerialize for HkpVehicleFrictionDescriptionAxisDescription {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
