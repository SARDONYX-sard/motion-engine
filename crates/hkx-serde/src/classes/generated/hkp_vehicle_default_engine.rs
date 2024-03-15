//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultEngine`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultEngine`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpVehicleEngine`/`0xda8c7d7d`
/// - signature: `0x56f8ca24`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultEngine {
    // `hkpVehicleEngine`(Parent class) has no fields

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
    /// -   name:`"minRPM"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minRPM")]
    MinRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"optRPM"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "optRPM")]
    OptRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxRPM"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxRPM")]
    MaxRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxTorque"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxTorque")]
    MaxTorque(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"torqueFactorAtMinRPM"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueFactorAtMinRPM")]
    TorqueFactorAtMinRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"torqueFactorAtMaxRPM"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueFactorAtMaxRPM")]
    TorqueFactorAtMaxRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"resistanceFactorAtMinRPM"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resistanceFactorAtMinRPM")]
    ResistanceFactorAtMinRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"resistanceFactorAtOptRPM"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resistanceFactorAtOptRPM")]
    ResistanceFactorAtOptRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"resistanceFactorAtMaxRPM"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resistanceFactorAtMaxRPM")]
    ResistanceFactorAtMaxRpm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"clutchSlipRPM"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "clutchSlipRPM")]
    ClutchSlipRpm(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultEngine, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("minRPM" => MinRpm(Primitive<f32>)),
    ("optRPM" => OptRpm(Primitive<f32>)),
    ("maxRPM" => MaxRpm(Primitive<f32>)),
    ("maxTorque" => MaxTorque(Primitive<f32>)),
    ("torqueFactorAtMinRPM" => TorqueFactorAtMinRpm(Primitive<f32>)),
    ("torqueFactorAtMaxRPM" => TorqueFactorAtMaxRpm(Primitive<f32>)),
    ("resistanceFactorAtMinRPM" => ResistanceFactorAtMinRpm(Primitive<f32>)),
    ("resistanceFactorAtOptRPM" => ResistanceFactorAtOptRpm(Primitive<f32>)),
    ("resistanceFactorAtMaxRPM" => ResistanceFactorAtMaxRpm(Primitive<f32>)),
    ("clutchSlipRPM" => ClutchSlipRpm(Primitive<f32>)),
}
