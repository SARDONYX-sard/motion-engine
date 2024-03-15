//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGroupFilter`
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
    /// # C++ Parent class(`hkpCollisionFilter`, parent: `hkReferencedObject`) field Info
    /// -   name:`"prepad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "prepad", default)]
    Prepad([Primitive<u32>; 2]),
    /// # C++ Parent class(`hkpCollisionFilter`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum hkpFilterType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<HkpFilterType>),
    /// # C++ Parent class(`hkpCollisionFilter`, parent: `hkReferencedObject`) field Info
    /// -   name:`"postpad"`
    /// -   type: `hkUint32[3]`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "postpad", default)]
    Postpad([Primitive<u32>; 3]),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"nextFreeSystemGroup"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextFreeSystemGroup", default)]
    NextFreeSystemGroup(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionLookupTable"`
    /// -   type: `hkUint32[32]`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionLookupTable", default)]
    CollisionLookupTable([Primitive<u32>; 32]),
    /// # C++ Class Fields Info
    /// -   name:`"pad256"`
    /// -   type: `hkVector4[4]`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad256", default)]
    Pad256([Vector4<f32>; 4]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGroupFilter, "@name",
    ("prepad" => Prepad([Primitive<u32>; 2])),
    ("type" => Type(Primitive<HkpFilterType>)),
    ("postpad" => Postpad([Primitive<u32>; 3])),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("nextFreeSystemGroup" => NextFreeSystemGroup(Primitive<i32>)),
    ("collisionLookupTable" => CollisionLookupTable([Primitive<u32>; 32])),
    ("pad256" => Pad256([Vector4<f32>; 4])),
}
