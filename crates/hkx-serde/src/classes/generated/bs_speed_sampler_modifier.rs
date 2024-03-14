//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSSpeedSamplerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSSpeedSamplerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xd297fda9`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsSpeedSamplerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"state"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "state")]
    State(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"direction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "direction")]
    Direction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"goalSpeed"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "goalSpeed")]
    GoalSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"speedOut"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "speedOut")]
    SpeedOut(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsSpeedSamplerModifier, "@name",
    ("state" => State(Primitive<i32>)),
    ("direction" => Direction(Primitive<f32>)),
    ("goalSpeed" => GoalSpeed(Primitive<f32>)),
    ("speedOut" => SpeedOut(Primitive<f32>)),
}
