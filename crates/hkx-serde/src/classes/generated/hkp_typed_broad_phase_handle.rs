//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTypedBroadPhaseHandle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTypedBroadPhaseHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpBroadPhaseHandle`/`0x940569dc`
/// - signature: `0xf4b0f799`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTypedBroadPhaseHandle {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `hkInt8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ownerOffset", skip_serializing)]
    OwnerOffset(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"objectQualityType"`
    /// -   type: `hkInt8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectQualityType")]
    ObjectQualityType(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTypedBroadPhaseHandle, "@name",
    ("type" => Type(Primitive<i8>)),
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("objectQualityType" => ObjectQualityType(Primitive<i8>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
}
