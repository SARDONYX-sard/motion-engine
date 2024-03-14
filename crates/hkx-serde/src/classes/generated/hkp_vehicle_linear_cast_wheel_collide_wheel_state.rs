//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpVehicleLinearCastWheelCollideWheelState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleLinearCastWheelCollideWheelState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x2a9acf98`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleLinearCastWheelCollideWheelState<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantom")]
    Phantom(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Transform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"to"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "to")]
    To(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollideWheelState<'de>, "@name",
    ("phantom" => Phantom(Cow<'de, str>)),
    ("shape" => Shape(Cow<'de, str>)),
    ("transform" => Transform(Transform<f32>)),
    ("to" => To(Vector4<f32>)),
}
