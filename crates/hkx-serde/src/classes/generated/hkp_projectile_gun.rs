//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpProjectileGun`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpProjectileGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpFirstPersonGun`/`0x852ab70b`
/// - signature: `0xb4f30148`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpProjectileGun<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"maxProjectiles"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxProjectiles")]
    MaxProjectiles(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"reloadTime"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reloadTime")]
    ReloadTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"reload"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "reload", skip_serializing)]
    Reload(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"projectiles"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "projectiles", skip_serializing)]
    Projectiles(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"destructionWorld"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "destructionWorld", skip_serializing)]
    DestructionWorld(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpProjectileGun<'de>, "@name",
    ("maxProjectiles" => MaxProjectiles(Primitive<i32>)),
    ("reloadTime" => ReloadTime(Primitive<f32>)),
    ("reload" => Reload(Primitive<f32>)),
    ("projectiles" => Projectiles(HkArrayRef<Cow<'de, str>>)),
    ("world" => World(Cow<'de, str>)),
    ("destructionWorld" => DestructionWorld(Cow<'de, str>)),
}
