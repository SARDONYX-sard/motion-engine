//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTypedBroadPhaseHandle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTypedBroadPhaseHandle {
    /// # C++ Parent class(`hkpBroadPhaseHandle` => parent: `None`) field Info
    /// -   name:`"id"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<u32>),

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
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
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
    ("id" => Id(Primitive<u32>)),
    ("type" => Type(Primitive<i8>)),
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("objectQualityType" => ObjectQualityType(Primitive<i8>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
}
