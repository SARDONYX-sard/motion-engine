//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbKeyframeBonesModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbKeyframeBonesModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x95f66629`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbKeyframeBonesModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"keyframeInfo"`
    /// -   type: `hkArray&lt;struct hkbKeyframeBonesModifierKeyframeInfo&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframeInfo")]
    KeyframeInfo(HkArrayClass<HkbKeyframeBonesModifierKeyframeInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"keyframedBonesList"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframedBonesList")]
    KeyframedBonesList(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbKeyframeBonesModifier<'de>, "@name",
    ("keyframeInfo" => KeyframeInfo(HkArrayClass<HkbKeyframeBonesModifierKeyframeInfo>)),
    ("keyframedBonesList" => KeyframedBonesList(Primitive<Cow<'de, str>>)),
}
