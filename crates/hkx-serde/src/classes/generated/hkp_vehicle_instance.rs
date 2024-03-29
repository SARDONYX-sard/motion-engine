//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleInstance`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 212
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x877bb579`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleInstance<'a> {
    /// # C++ Parent class(`hkpUnaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entity"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entity")]
    Entity(Primitive<Cow<'a, str>>),

    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"island"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "island", skip_serializing)]
    Island(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),

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
    //
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkpVehicleData*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"driverInput"`
    /// -   type: `struct hkpVehicleDriverInput*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "driverInput")]
    DriverInput(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"steering"`
    /// -   type: `struct hkpVehicleSteering*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "steering")]
    Steering(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"engine"`
    /// -   type: `struct hkpVehicleEngine*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "engine")]
    Engine(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"transmission"`
    /// -   type: `struct hkpVehicleTransmission*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transmission")]
    Transmission(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"brake"`
    /// -   type: `struct hkpVehicleBrake*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "brake")]
    Brake(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"suspension"`
    /// -   type: `struct hkpVehicleSuspension*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "suspension")]
    Suspension(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"aerodynamics"`
    /// -   type: `struct hkpVehicleAerodynamics*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aerodynamics")]
    Aerodynamics(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelCollide"`
    /// -   type: `struct hkpVehicleWheelCollide*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollide")]
    WheelCollide(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"tyreMarks"`
    /// -   type: `struct hkpTyremarksInfo*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tyreMarks")]
    TyreMarks(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"velocityDamper"`
    /// -   type: `struct hkpVehicleVelocityDamper*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocityDamper")]
    VelocityDamper(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelsInfo"`
    /// -   type: `hkArray<struct hkpVehicleInstanceWheelInfo>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsInfo")]
    WheelsInfo(HkArrayClass<HkpVehicleInstanceWheelInfo<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"frictionStatus"`
    /// -   type: `struct hkpVehicleFrictionStatus`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frictionStatus")]
    FrictionStatus(SingleClass<HkpVehicleFrictionStatus>),
    /// # C++ Class Fields Info
    /// -   name:`"deviceStatus"`
    /// -   type: `struct hkpVehicleDriverInputStatus*`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deviceStatus")]
    DeviceStatus(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"isFixed"`
    /// -   type: `hkArray<hkBool>`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isFixed")]
    IsFixed(HkArrayRef<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelsTimeSinceMaxPedalInput"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsTimeSinceMaxPedalInput")]
    WheelsTimeSinceMaxPedalInput(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"tryingToReverse"`
    /// -   type: `hkBool`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tryingToReverse")]
    TryingToReverse(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"torque"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torque")]
    Torque(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rpm"`
    /// -   type: `hkReal`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rpm")]
    Rpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"mainSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mainSteeringAngle")]
    MainSteeringAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelsSteeringAngle"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsSteeringAngle")]
    WheelsSteeringAngle(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isReversing"`
    /// -   type: `hkBool`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isReversing")]
    IsReversing(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"currentGear"`
    /// -   type: `hkInt8`
    /// - offset: 205
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentGear")]
    CurrentGear(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"delayed"`
    /// -   type: `hkBool`
    /// - offset: 206
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayed")]
    Delayed(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"clutchDelayCountdown"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "clutchDelayCountdown")]
    ClutchDelayCountdown(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleInstance<'de>, "@name",
    ("entity" => Entity(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("driverInput" => DriverInput(Primitive<Cow<'de, str>>)),
    ("steering" => Steering(Primitive<Cow<'de, str>>)),
    ("engine" => Engine(Primitive<Cow<'de, str>>)),
    ("transmission" => Transmission(Primitive<Cow<'de, str>>)),
    ("brake" => Brake(Primitive<Cow<'de, str>>)),
    ("suspension" => Suspension(Primitive<Cow<'de, str>>)),
    ("aerodynamics" => Aerodynamics(Primitive<Cow<'de, str>>)),
    ("wheelCollide" => WheelCollide(Primitive<Cow<'de, str>>)),
    ("tyreMarks" => TyreMarks(Primitive<Cow<'de, str>>)),
    ("velocityDamper" => VelocityDamper(Primitive<Cow<'de, str>>)),
    ("wheelsInfo" => WheelsInfo(HkArrayClass<HkpVehicleInstanceWheelInfo<'de>>)),
    ("frictionStatus" => FrictionStatus(SingleClass<HkpVehicleFrictionStatus>)),
    ("deviceStatus" => DeviceStatus(Primitive<Cow<'de, str>>)),
    ("isFixed" => IsFixed(HkArrayRef<bool>)),
    ("wheelsTimeSinceMaxPedalInput" => WheelsTimeSinceMaxPedalInput(Primitive<f32>)),
    ("tryingToReverse" => TryingToReverse(Primitive<bool>)),
    ("torque" => Torque(Primitive<f32>)),
    ("rpm" => Rpm(Primitive<f32>)),
    ("mainSteeringAngle" => MainSteeringAngle(Primitive<f32>)),
    ("wheelsSteeringAngle" => WheelsSteeringAngle(HkArrayNum<f32>)),
    ("isReversing" => IsReversing(Primitive<bool>)),
    ("currentGear" => CurrentGear(Primitive<i8>)),
    ("delayed" => Delayed(Primitive<bool>)),
    ("clutchDelayCountdown" => ClutchDelayCountdown(Primitive<f32>)),
}

impl ByteDeSerialize for HkpVehicleInstance<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
