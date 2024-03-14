//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbClipGeneratorEcho`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbClipGeneratorEcho`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x750edf40`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipGeneratorEcho {
    /// # C++ Class Fields Info
    /// -   name:`"offsetLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "offsetLocalTime")]
    OffsetLocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"weight"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weight")]
    Weight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dwdt"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dwdt")]
    Dwdt(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorEcho, "@name",
    ("offsetLocalTime" => OffsetLocalTime(Primitive<f32>)),
    ("weight" => Weight(Primitive<f32>)),
    ("dwdt" => Dwdt(Primitive<f32>)),
}
