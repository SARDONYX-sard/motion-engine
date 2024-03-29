//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleLinearCastWheelCollideWheelState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleLinearCastWheelCollideWheelState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x2a9acf98`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleLinearCastWheelCollideWheelState<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantom")]
    Phantom(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"to"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "to")]
    To(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollideWheelState<'de>, "@name",
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
    ("to" => To(Primitive<Vector4<f32>>)),
}

impl ByteDeSerialize for HkpVehicleLinearCastWheelCollideWheelState<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
