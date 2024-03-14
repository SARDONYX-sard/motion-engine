//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpVehicleDefaultAnalogDriverInput`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultAnalogDriverInput`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkpVehicleDriverInput`/`0xda8c7d7d`
/// - signature: `0x123a5d50`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultAnalogDriverInput {
    /// # C++ Class Fields Info
    /// -   name:`"slopeChangePointX"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "slopeChangePointX")]
    SlopeChangePointX(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"initialSlope"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialSlope")]
    InitialSlope(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"deadZone"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deadZone")]
    DeadZone(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"autoReverse"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoReverse")]
    AutoReverse(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultAnalogDriverInput, "@name",
    ("slopeChangePointX" => SlopeChangePointX(Primitive<f32>)),
    ("initialSlope" => InitialSlope(Primitive<f32>)),
    ("deadZone" => DeadZone(Primitive<f32>)),
    ("autoReverse" => AutoReverse(Primitive<bool>)),
}
