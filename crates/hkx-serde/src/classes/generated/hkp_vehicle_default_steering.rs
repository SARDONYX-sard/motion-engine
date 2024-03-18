//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSteering`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleDefaultSteering`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkpVehicleSteering`/`0xda8c7d7d`
/// - signature: `0x8f0411c8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultSteering {
    // C++ Parent class(`hkpVehicleSteering` => parent: `hkReferencedObject`) has no fields

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"maxSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSteeringAngle")]
    MaxSteeringAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSpeedFullSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSpeedFullSteeringAngle")]
    MaxSpeedFullSteeringAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"doesWheelSteer"`
    /// -   type: `hkArray<hkBool>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "doesWheelSteer")]
    DoesWheelSteer(HkArrayRef<Primitive<bool>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSteering, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("maxSteeringAngle" => MaxSteeringAngle(Primitive<f32>)),
    ("maxSpeedFullSteeringAngle" => MaxSpeedFullSteeringAngle(Primitive<f32>)),
    ("doesWheelSteer" => DoesWheelSteer(HkArrayRef<Primitive<bool>>)),
}
