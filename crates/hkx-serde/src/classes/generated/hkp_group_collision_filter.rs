//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGroupCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpGroupCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 180
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0x5cc01561`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGroupCollisionFilter {
    /// # C++ Class Fields Info
    /// -   name:`"noGroupCollisionEnabled"`
    /// -   type: `hkBool`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "noGroupCollisionEnabled")]
    NoGroupCollisionEnabled(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionGroups"`
    /// -   type: `hkUint32[32]`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionGroups")]
    CollisionGroups([Primitive<u32>; 32]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGroupCollisionFilter, "@name",
    ("noGroupCollisionEnabled" => NoGroupCollisionEnabled(Primitive<bool>)),
    ("collisionGroups" => CollisionGroups([Primitive<u32>; 32])),
}
