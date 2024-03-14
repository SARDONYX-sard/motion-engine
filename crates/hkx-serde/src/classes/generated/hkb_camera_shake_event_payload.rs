//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCameraShakeEventPayload`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCameraShakeEventPayload`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: true
/// -    parent: `hkbEventPayload`/`0xda8c7d7d`
/// - signature: `0x64136982`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCameraShakeEventPayload {
    /// # C++ Class Fields Info
    /// -   name:`"amplitude"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "amplitude")]
    Amplitude(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"halfLife"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "halfLife")]
    HalfLife(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCameraShakeEventPayload, "@name",
    ("amplitude" => Amplitude(Primitive<f32>)),
    ("halfLife" => HalfLife(Primitive<f32>)),
}
