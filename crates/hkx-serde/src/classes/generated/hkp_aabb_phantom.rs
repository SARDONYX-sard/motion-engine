//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAabbPhantom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpAabbPhantom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 224
/// -    vtable: true
/// -    parent: `hkpPhantom`/`0x9b7e6f86`
/// - signature: `0x2c5189dd`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAabbPhantom<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"aabb"`
    /// -   type: `struct hkAabb`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabb")]
    Aabb(HkAabb),
    /// # C++ Class Fields Info
    /// -   name:`"overlappingCollidables"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "overlappingCollidables", skip_serializing)]
    OverlappingCollidables(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "orderDirty", skip_serializing)]
    OrderDirty(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAabbPhantom<'de>, "@name",
    ("aabb" => Aabb(HkAabb)),
    ("overlappingCollidables" => OverlappingCollidables(HkArrayRef<Cow<'de, str>>)),
    ("orderDirty" => OrderDirty(Primitive<bool>)),
}
