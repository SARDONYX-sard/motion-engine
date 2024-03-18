//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x7be5bed1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dampingCompression"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampingCompression")]
    DampingCompression(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dampingRelaxation"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampingRelaxation")]
    DampingRelaxation(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters, "@name",
    ("strength" => Strength(Primitive<f32>)),
    ("dampingCompression" => DampingCompression(Primitive<f32>)),
    ("dampingRelaxation" => DampingRelaxation(Primitive<f32>)),
}
