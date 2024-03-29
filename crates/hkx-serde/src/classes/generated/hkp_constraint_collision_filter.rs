//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintCollisionFilter<'a> {
    /// # C++ Parent class(`hkpPairCollisionFilter` => parent: `hkpCollisionFilter`) field Info
    /// -   name:`"disabledPairs"`
    /// -   type: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "disabledPairs", skip_serializing)]
    DisabledPairs(SingleClass<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>),
    /// # C++ Parent class(`hkpPairCollisionFilter` => parent: `hkpCollisionFilter`) field Info
    /// -   name:`"childFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childFilter")]
    ChildFilter(Primitive<Cow<'a, str>>),

    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"prepad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "prepad")]
    Prepad(CStyleArray<[u32; 2]>),
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
    Postpad(CStyleArray<[u32; 3]>),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),
    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintCollisionFilter<'de>, "@name",
    ("disabledPairs" => DisabledPairs(SingleClass<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'de>>)),
    ("childFilter" => ChildFilter(Primitive<Cow<'de, str>>)),
    ("prepad" => Prepad(CStyleArray<[u32; 2]>)),
    ("type" => Type(Primitive<HkpFilterType>)),
    ("postpad" => Postpad(CStyleArray<[u32; 3]>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}
