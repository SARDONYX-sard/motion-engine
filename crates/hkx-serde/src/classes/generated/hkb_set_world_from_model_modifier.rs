//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbSetWorldFromModelModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSetWorldFromModelModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xafcfa211`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetWorldFromModelModifier {
    /// # C++ Class Fields Info
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translation")]
    Translation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"setTranslation"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setTranslation")]
    SetTranslation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"setRotation"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setRotation")]
    SetRotation(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetWorldFromModelModifier, "@name",
    ("translation" => Translation(Vector4<f32>)),
    ("rotation" => Rotation(Quaternion<f32>)),
    ("setTranslation" => SetTranslation(Primitive<bool>)),
    ("setRotation" => SetRotation(Primitive<bool>)),
}
