//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultAerodynamics`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultAerodynamics`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpVehicleAerodynamics`/`0xda8c7d7d`
/// - signature: `0x42fc5bbd`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultAerodynamics {
    /// # C++ Class Fields Info
    /// -   name:`"airDensity"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "airDensity")]
    AirDensity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"frontalArea"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frontalArea")]
    FrontalArea(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dragCoefficient"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dragCoefficient")]
    DragCoefficient(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"liftCoefficient"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "liftCoefficient")]
    LiftCoefficient(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extraGravityws"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraGravityws")]
    ExtraGravityws(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultAerodynamics, "@name",
    ("airDensity" => AirDensity(Primitive<f32>)),
    ("frontalArea" => FrontalArea(Primitive<f32>)),
    ("dragCoefficient" => DragCoefficient(Primitive<f32>)),
    ("liftCoefficient" => LiftCoefficient(Primitive<f32>)),
    ("extraGravityws" => ExtraGravityws(Vector4<f32>)),
}
