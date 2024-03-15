//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbKeyframeBonesModifierKeyframeInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbKeyframeBonesModifierKeyframeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x72deb7a6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbKeyframeBonesModifierKeyframeInfo {
    /// # C++ Class Fields Info
    /// -   name:`"keyframedPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframedPosition", default)]
    KeyframedPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"keyframedRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframedRotation", default)]
    KeyframedRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex", default)]
    BoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"isValid"`
    /// -   type: `hkBool`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isValid", default)]
    IsValid(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbKeyframeBonesModifierKeyframeInfo, "@name",
    ("keyframedPosition" => KeyframedPosition(Vector4<f32>)),
    ("keyframedRotation" => KeyframedRotation(Quaternion<f32>)),
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("isValid" => IsValid(Primitive<bool>)),
}
