//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSuspension`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultSuspension {
    /// # C++ Class Fields Info
    /// -   name:`"wheelSpringParams"`
    /// -   type: `hkArray&lt;struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelSpringParams")]
    WheelSpringParams(HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSuspension, "@name",
    ("wheelSpringParams" => WheelSpringParams(HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>)),
}
