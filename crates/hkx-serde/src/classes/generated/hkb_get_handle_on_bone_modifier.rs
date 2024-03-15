//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGetHandleOnBoneModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbGetHandleOnBoneModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x50c34a17`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGetHandleOnBoneModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"handleOut"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleOut")]
    HandleOut(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollBoneIndex")]
    RagdollBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetHandleOnBoneModifier<'de>, "@name",
    ("handleOut" => HandleOut(Primitive<Cow<'de, str>>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'de, str>>)),
    ("ragdollBoneIndex" => RagdollBoneIndex(Primitive<i16>)),
    ("animationBoneIndex" => AnimationBoneIndex(Primitive<i16>)),
}
