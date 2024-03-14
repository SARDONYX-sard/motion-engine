//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCombineTransformsModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCombineTransformsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xfd1f0b79`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCombineTransformsModifier {
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
    /// # C++ Class Fields Info
    /// -   name:`"leftTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leftTranslation")]
    LeftTranslation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"leftRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leftRotation")]
    LeftRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rightTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rightTranslation")]
    RightTranslation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rightRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rightRotation")]
    RightRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"invertLeftTransform"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invertLeftTransform")]
    InvertLeftTransform(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"invertRightTransform"`
    /// -   type: `hkBool`
    /// - offset: 145
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invertRightTransform")]
    InvertRightTransform(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"invertResult"`
    /// -   type: `hkBool`
    /// - offset: 146
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invertResult")]
    InvertResult(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCombineTransformsModifier, "@name",
    ("translationOut" => TranslationOut(Vector4<f32>)),
    ("rotationOut" => RotationOut(Quaternion<f32>)),
    ("leftTranslation" => LeftTranslation(Vector4<f32>)),
    ("leftRotation" => LeftRotation(Quaternion<f32>)),
    ("rightTranslation" => RightTranslation(Vector4<f32>)),
    ("rightRotation" => RightRotation(Quaternion<f32>)),
    ("invertLeftTransform" => InvertLeftTransform(Primitive<bool>)),
    ("invertRightTransform" => InvertRightTransform(Primitive<bool>)),
    ("invertResult" => InvertResult(Primitive<bool>)),
}
