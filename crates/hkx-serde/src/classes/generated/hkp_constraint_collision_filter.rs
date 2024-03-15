//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkpPairCollisionFilter`/`0x4abc140e`
/// - signature: `0xc3b577b1`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintCollisionFilter {
    /// # C++ Parent class(`hkpPairCollisionFilter`, parent: `hkpCollisionFilter`) field Info
    /// -   name:`"disabledPairs"`
    /// -   type: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "disabledPairs", default, skip_serializing)]
    DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType),
    /// # C++ Parent class(`hkpPairCollisionFilter`, parent: `hkpCollisionFilter`) field Info
    /// -   name:`"childFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childFilter", default)]
    ChildFilter(Primitive<Cow<'a, str>>),

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

}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintCollisionFilter, "@name",
    ("disabledPairs" => DisabledPairs(HkpPairCollisionFilterMapPairFilterKeyOverrideType)),
    ("childFilter" => ChildFilter(Primitive<Cow<'de, str>>)),
    ("prepad" => Prepad([Primitive<u32>; 2])),
    ("type" => Type(Primitive<HkpFilterType>)),
    ("postpad" => Postpad([Primitive<u32>; 3])),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}
