//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPairCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPairCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0x4abc140e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPairCollisionFilter<'a> {
    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"prepad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "prepad")]
    Prepad(CStyleArray<u32, 2>),
    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum hkpFilterType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<HkpFilterType>),
    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"postpad"`
    /// -   type: `hkUint32[3]`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "postpad")]
    Postpad(CStyleArray<u32, 3>),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"disabledPairs"`
    /// -   type: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "disabledPairs", skip_serializing)]
    DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"childFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childFilter")]
    ChildFilter(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPairCollisionFilter<'de>, "@name",
    ("prepad" => Prepad(CStyleArray<u32, 2>)),
    ("type" => Type(Primitive<HkpFilterType>)),
    ("postpad" => Postpad(CStyleArray<u32, 3>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("disabledPairs" => DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType<'de>)),
    ("childFilter" => ChildFilter(Primitive<Cow<'de, str>>)),
}
