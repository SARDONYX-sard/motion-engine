//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionStatus`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleFrictionStatus`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: false
/// - signature: `0x1c076a84`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionStatus {
    /// # C++ Class Fields Info
    /// -   name:`"axis"`
    /// -   type: `struct hkpVehicleFrictionStatusAxisStatus[2]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axis")]
    Axis(CStyleArrayClass<HkpVehicleFrictionStatusAxisStatus, 2>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionStatus, "@name",
    ("axis" => Axis(CStyleArrayClass<HkpVehicleFrictionStatusAxisStatus, 2>)),
}
