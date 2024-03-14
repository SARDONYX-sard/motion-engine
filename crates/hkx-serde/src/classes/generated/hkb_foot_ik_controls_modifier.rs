//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkControlsModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbFootIkControlsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xe5b6f544`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkControlsModifier {
    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbFootIkControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbFootIkControlData),
    /// # C++ Class Fields Info
    /// -   name:`"legs"`
    /// -   type: `hkArray&lt;struct hkbFootIkControlsModifierLeg&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(HkArrayClass<HkbFootIkControlsModifierLeg>),
    /// # C++ Class Fields Info
    /// -   name:`"errorOutTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOutTranslation")]
    ErrorOutTranslation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(Quaternion<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkControlsModifier, "@name",
    ("controlData" => ControlData(HkbFootIkControlData)),
    ("legs" => Legs(HkArrayClass<HkbFootIkControlsModifierLeg>)),
    ("errorOutTranslation" => ErrorOutTranslation(Vector4<f32>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(Quaternion<f32>)),
}
