//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultTransmission`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultTransmission`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkpVehicleTransmission`/`0xda8c7d7d`
/// - signature: `0x235d5d6b`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultTransmission {
    // `hkpVehicleTransmission`(Parent class) has no fields

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"downshiftRPM"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "downshiftRPM", default)]
    DownshiftRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"upshiftRPM"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upshiftRPM", default)]
    UpshiftRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"primaryTransmissionRatio"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "primaryTransmissionRatio", default)]
    PrimaryTransmissionRatio(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"clutchDelayTime"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "clutchDelayTime", default)]
    ClutchDelayTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"reverseGearRatio"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseGearRatio", default)]
    ReverseGearRatio(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"gearsRatio"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gearsRatio", default)]
    GearsRatio(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"wheelsTorqueRatio"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelsTorqueRatio", default)]
    WheelsTorqueRatio(HkArrayRef<Primitive<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultTransmission, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("downshiftRPM" => DownshiftRpm(Primitive<f32>)),
    ("upshiftRPM" => UpshiftRpm(Primitive<f32>)),
    ("primaryTransmissionRatio" => PrimaryTransmissionRatio(Primitive<f32>)),
    ("clutchDelayTime" => ClutchDelayTime(Primitive<f32>)),
    ("reverseGearRatio" => ReverseGearRatio(Primitive<f32>)),
    ("gearsRatio" => GearsRatio(HkArrayRef<Primitive<f32>>)),
    ("wheelsTorqueRatio" => WheelsTorqueRatio(HkArrayRef<Primitive<f32>>)),
}
