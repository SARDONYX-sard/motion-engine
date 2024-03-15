//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 416
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x173feb43`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleData {
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numWheels"`
    /// -   type: `hkInt8`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numWheels")]
    NumWheels(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"chassisOrientation"`
    /// -   type: `hkRotation`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisOrientation")]
    ChassisOrientation(Rotation<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"torqueRollFactor"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueRollFactor")]
    TorqueRollFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"torquePitchFactor"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torquePitchFactor")]
    TorquePitchFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"torqueYawFactor"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueYawFactor")]
    TorqueYawFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extraTorqueFactor"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraTorqueFactor")]
    ExtraTorqueFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVelocityForPositionalFriction"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVelocityForPositionalFriction")]
    MaxVelocityForPositionalFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"chassisUnitInertiaYaw"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisUnitInertiaYaw")]
    ChassisUnitInertiaYaw(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"chassisUnitInertiaRoll"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisUnitInertiaRoll")]
    ChassisUnitInertiaRoll(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"chassisUnitInertiaPitch"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisUnitInertiaPitch")]
    ChassisUnitInertiaPitch(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"frictionEqualizer"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionEqualizer")]
    FrictionEqualizer(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"normalClippingAngleCos"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalClippingAngleCos")]
    NormalClippingAngleCos(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxFrictionSolverMassRatio"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFrictionSolverMassRatio")]
    MaxFrictionSolverMassRatio(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelParams"`
    /// -   type: `hkArray&lt;struct hkpVehicleDataWheelComponentParams&gt;`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelParams")]
    WheelParams(HkArrayClass<HkpVehicleDataWheelComponentParams>),
    /// # C++ Class Fields Info
    /// -   name:`"numWheelsPerAxle"`
    /// -   type: `hkArray&lt;hkInt8&gt;`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numWheelsPerAxle")]
    NumWheelsPerAxle(HkArrayRef<Primitive<i8>>),
    /// # C++ Class Fields Info
    /// -   name:`"frictionDescription"`
    /// -   type: `struct hkpVehicleFrictionDescription`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionDescription")]
    FrictionDescription(HkpVehicleFrictionDescription),
    /// # C++ Class Fields Info
    /// -   name:`"chassisFrictionInertiaInvDiag"`
    /// -   type: `hkVector4`
    /// - offset: 384
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisFrictionInertiaInvDiag")]
    ChassisFrictionInertiaInvDiag(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"alreadyInitialised"`
    /// -   type: `hkBool`
    /// - offset: 400
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alreadyInitialised")]
    AlreadyInitialised(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleData, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("gravity" => Gravity(Vector4<f32>)),
    ("numWheels" => NumWheels(Primitive<i8>)),
    ("chassisOrientation" => ChassisOrientation(Rotation<f32>)),
    ("torqueRollFactor" => TorqueRollFactor(Primitive<f32>)),
    ("torquePitchFactor" => TorquePitchFactor(Primitive<f32>)),
    ("torqueYawFactor" => TorqueYawFactor(Primitive<f32>)),
    ("extraTorqueFactor" => ExtraTorqueFactor(Primitive<f32>)),
    ("maxVelocityForPositionalFriction" => MaxVelocityForPositionalFriction(Primitive<f32>)),
    ("chassisUnitInertiaYaw" => ChassisUnitInertiaYaw(Primitive<f32>)),
    ("chassisUnitInertiaRoll" => ChassisUnitInertiaRoll(Primitive<f32>)),
    ("chassisUnitInertiaPitch" => ChassisUnitInertiaPitch(Primitive<f32>)),
    ("frictionEqualizer" => FrictionEqualizer(Primitive<f32>)),
    ("normalClippingAngleCos" => NormalClippingAngleCos(Primitive<f32>)),
    ("maxFrictionSolverMassRatio" => MaxFrictionSolverMassRatio(Primitive<f32>)),
    ("wheelParams" => WheelParams(HkArrayClass<HkpVehicleDataWheelComponentParams>)),
    ("numWheelsPerAxle" => NumWheelsPerAxle(HkArrayRef<Primitive<i8>>)),
    ("frictionDescription" => FrictionDescription(HkpVehicleFrictionDescription)),
    ("chassisFrictionInertiaInvDiag" => ChassisFrictionInertiaInvDiag(Vector4<f32>)),
    ("alreadyInitialised" => AlreadyInitialised(Primitive<bool>)),
}
