//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSPassByTargetTriggerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSPassByTargetTriggerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x703d7b66`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsPassByTargetTriggerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"movementDirection"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "movementDirection")]
    MovementDirection(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"triggerEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerEvent")]
    TriggerEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"targetPassed"`
    /// -   type: `hkBool`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "targetPassed", skip_serializing)]
    TargetPassed(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsPassByTargetTriggerModifier, "@name",
    ("targetPosition" => TargetPosition(Vector4<f32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("movementDirection" => MovementDirection(Vector4<f32>)),
    ("triggerEvent" => TriggerEvent(HkbEventProperty)),
    ("targetPassed" => TargetPassed(Primitive<bool>)),
}
