//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbGetWorldFromModelModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbGetWorldFromModelModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x873fc6f7`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGetWorldFromModelModifier {
    /// # C++ Class Fields Info
    /// -   name:`"translationOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationOut")]
    TranslationOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationOut")]
    RotationOut(Quaternion<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetWorldFromModelModifier, "@name",
    ("translationOut" => TranslationOut(Vector4<f32>)),
    ("rotationOut" => RotationOut(Quaternion<f32>)),
}
