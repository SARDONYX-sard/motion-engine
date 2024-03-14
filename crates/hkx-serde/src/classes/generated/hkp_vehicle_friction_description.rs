//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionDescription`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleFrictionDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x1034549a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionDescription {
    /// # C++ Class Fields Info
    /// -   name:`"wheelDistance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDistance")]
    WheelDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"chassisMassInv"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisMassInv")]
    ChassisMassInv(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"axleDescr"`
    /// -   type: `struct hkpVehicleFrictionDescriptionAxisDescription[2]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axleDescr")]
    AxleDescr([HkpVehicleFrictionDescriptionAxisDescription; 2]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescription, "@name",
    ("wheelDistance" => WheelDistance(Primitive<f32>)),
    ("chassisMassInv" => ChassisMassInv(Primitive<f32>)),
    ("axleDescr" => AxleDescr([HkpVehicleFrictionDescriptionAxisDescription; 2])),
}
