//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpVehicleFrictionStatus`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleFrictionStatus`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x1c076a84`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionStatus {
    /// # C++ Class Fields Info
    /// -   name:`"axis"`
    /// -   type: `struct hkpVehicleFrictionStatusAxisStatus[2]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axis")]
    Axis([HkpVehicleFrictionStatusAxisStatus; 2]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionStatus, "@name",
    ("axis" => Axis([HkpVehicleFrictionStatusAxisStatus; 2])),
}
