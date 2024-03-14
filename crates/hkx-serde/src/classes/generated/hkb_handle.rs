//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbHandle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd8b6401c`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandle<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"frame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frame")]
    Frame(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"rigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBody")]
    RigidBody(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"character"`
    /// -   type: `struct hkbCharacter*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "character")]
    Character(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandle<'de>, "@name",
    ("frame" => Frame(Cow<'de, str>)),
    ("rigidBody" => RigidBody(Cow<'de, str>)),
    ("character" => Character(Cow<'de, str>)),
    ("animationBoneIndex" => AnimationBoneIndex(Primitive<i16>)),
}
