//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbDelayedModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbDelayedModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbModifierWrapper`/`0x3697e044`
/// - signature: `0x8e101a7a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDelayedModifier {
    /// # C++ Class Fields Info
    /// -   name:`"delaySeconds"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delaySeconds")]
    DelaySeconds(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"durationSeconds"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "durationSeconds")]
    DurationSeconds(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"secondsElapsed"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "secondsElapsed", skip_serializing)]
    SecondsElapsed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDelayedModifier, "@name",
    ("delaySeconds" => DelaySeconds(Primitive<f32>)),
    ("durationSeconds" => DurationSeconds(Primitive<f32>)),
    ("secondsElapsed" => SecondsElapsed(Primitive<f32>)),
    ("isActive" => IsActive(Primitive<bool>)),
}
