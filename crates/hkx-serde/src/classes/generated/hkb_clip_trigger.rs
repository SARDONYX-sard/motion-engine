//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipTrigger`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbClipTrigger`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x7eb45cea`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipTrigger {
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime", default)]
    LocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event", default)]
    Event(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"relativeToEndOfClip"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeToEndOfClip", default)]
    RelativeToEndOfClip(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"acyclic"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "acyclic", default)]
    Acyclic(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isAnnotation"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isAnnotation", default)]
    IsAnnotation(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipTrigger, "@name",
    ("localTime" => LocalTime(Primitive<f32>)),
    ("event" => Event(HkbEventProperty)),
    ("relativeToEndOfClip" => RelativeToEndOfClip(Primitive<bool>)),
    ("acyclic" => Acyclic(Primitive<bool>)),
    ("isAnnotation" => IsAnnotation(Primitive<bool>)),
}
