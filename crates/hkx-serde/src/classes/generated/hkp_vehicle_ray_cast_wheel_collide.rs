//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleRayCastWheelCollide`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleRayCastWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpVehicleWheelCollide`/`0x4a50fcb`
/// - signature: `0x41efd9e3`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleRayCastWheelCollide<'a> {
    /// # C++ Parent class(`hkpVehicleWheelCollide` => parent: `hkReferencedObject`) field Info
    /// -   name:`"alreadyUsed"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alreadyUsed")]
    AlreadyUsed(Primitive<bool>),
    /// # C++ Parent class(`hkpVehicleWheelCollide` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

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
    /// -   name:`"wheelCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelCollisionFilterInfo")]
    WheelCollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantom")]
    Phantom(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rejectRayChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rejectRayChassisListener")]
    RejectRayChassisListener(SingleClass<HkpRejectChassisListener<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleRayCastWheelCollide<'de>, "@name",
    ("alreadyUsed" => AlreadyUsed(Primitive<bool>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(Primitive<u32>)),
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("rejectRayChassisListener" => RejectRayChassisListener(SingleClass<HkpRejectChassisListener<'de>>)),
}
