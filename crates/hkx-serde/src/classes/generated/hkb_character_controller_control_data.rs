//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterControllerControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacterControllerControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x5b6c03d9`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerControlData {
    /// # C++ Class Fields Info
    /// -   name:`"desiredVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "desiredVelocity", default)]
    DesiredVelocity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalGain", default)]
    VerticalGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"horizontalCatchUpGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "horizontalCatchUpGain", default)]
    HorizontalCatchUpGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalSeparation", default)]
    MaxVerticalSeparation(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxHorizontalSeparation", default)]
    MaxHorizontalSeparation(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerControlData, "@name",
    ("desiredVelocity" => DesiredVelocity(Vector4<f32>)),
    ("verticalGain" => VerticalGain(Primitive<f32>)),
    ("horizontalCatchUpGain" => HorizontalCatchUpGain(Primitive<f32>)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(Primitive<f32>)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(Primitive<f32>)),
}
