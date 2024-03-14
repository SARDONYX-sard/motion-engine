//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbFootIkControlsModifierLeg`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbFootIkControlsModifierLeg`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x9e17091a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkControlsModifierLeg {
    /// # C++ Class Fields Info
    /// -   name:`"groundPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundPosition")]
    GroundPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ungroundedEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ungroundedEvent")]
    UngroundedEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"verticalError"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalError")]
    VerticalError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"hitSomething"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hitSomething")]
    HitSomething(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isPlantedMS"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isPlantedMS")]
    IsPlantedMs(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkControlsModifierLeg, "@name",
    ("groundPosition" => GroundPosition(Vector4<f32>)),
    ("ungroundedEvent" => UngroundedEvent(HkbEventProperty)),
    ("verticalError" => VerticalError(Primitive<f32>)),
    ("hitSomething" => HitSomething(Primitive<bool>)),
    ("isPlantedMS" => IsPlantedMs(Primitive<bool>)),
}
