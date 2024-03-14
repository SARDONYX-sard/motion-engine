//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpGroupFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpGroupFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0x65ee88e4`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGroupFilter {
    /// # C++ Class Fields Info
    /// -   name:`"nextFreeSystemGroup"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextFreeSystemGroup")]
    NextFreeSystemGroup(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionLookupTable"`
    /// -   type: `hkUint32[32]`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionLookupTable")]
    CollisionLookupTable([Primitive<u32>; 32]),
    /// # C++ Class Fields Info
    /// -   name:`"pad256"`
    /// -   type: `hkVector4[4]`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad256")]
    Pad256([Vector4<f32>; 4]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGroupFilter, "@name",
    ("nextFreeSystemGroup" => NextFreeSystemGroup(Primitive<i32>)),
    ("collisionLookupTable" => CollisionLookupTable([Primitive<u32>; 32])),
    ("pad256" => Pad256([Vector4<f32>; 4])),
}
