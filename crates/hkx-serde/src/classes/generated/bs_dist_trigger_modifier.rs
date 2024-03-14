//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSDistTriggerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSDistTriggerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xb34d2bbd`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsDistTriggerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"distance"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distance")]
    Distance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"distanceTrigger"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distanceTrigger")]
    DistanceTrigger(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"triggerEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerEvent")]
    TriggerEvent(HkbEventProperty),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsDistTriggerModifier, "@name",
    ("targetPosition" => TargetPosition(Vector4<f32>)),
    ("distance" => Distance(Primitive<f32>)),
    ("distanceTrigger" => DistanceTrigger(Primitive<f32>)),
    ("triggerEvent" => TriggerEvent(HkbEventProperty)),
}
