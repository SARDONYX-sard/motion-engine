//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSuspension`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleDefaultSuspension`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpVehicleSuspension`/`0xaf5056fa`
/// - signature: `0x21735a24`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultSuspension {
    /// # C++ Parent class(`hkpVehicleSuspension` => parent: `hkReferencedObject`) field Info
    /// -   name:`"wheelParams"`
    /// -   type: `hkArray<struct hkpVehicleSuspensionSuspensionWheelParameters>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelParams")]
    WheelParams(HkArrayClass<HkpVehicleSuspensionSuspensionWheelParameters>),

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
    /// -   name:`"wheelSpringParams"`
    /// -   type: `hkArray<struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelSpringParams")]
    WheelSpringParams(HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSuspension, "@name",
    ("wheelParams" => WheelParams(HkArrayClass<HkpVehicleSuspensionSuspensionWheelParameters>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wheelSpringParams" => WheelSpringParams(HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>)),
}
