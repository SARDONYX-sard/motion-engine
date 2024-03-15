//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMoppBvTreeShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMoppBvTreeShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkMoppBvTreeShapeBase`/`0x7c338c66`
/// - signature: `0x90b29d39`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMoppBvTreeShape<'a> {
    /// # C++ Parent class(`hkMoppBvTreeShapeBase`, parent: `hkpBvTreeShape`) field Info
    /// -   name:`"code"`
    /// -   type: `struct hkpMoppCode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "code")]
    Code(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkMoppBvTreeShapeBase`, parent: `hkpBvTreeShape`) field Info
    /// -   name:`"moppData"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "moppData", skip_serializing)]
    MoppData(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkMoppBvTreeShapeBase`, parent: `hkpBvTreeShape`) field Info
    /// -   name:`"moppDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "moppDataSize", skip_serializing)]
    MoppDataSize(Primitive<u32>),
    /// # C++ Parent class(`hkMoppBvTreeShapeBase`, parent: `hkpBvTreeShape`) field Info
    /// -   name:`"codeInfoCopy"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "codeInfoCopy", skip_serializing)]
    CodeInfoCopy(Vector4<f32>),

    /// # C++ Parent class(`hkpBvTreeShape`, parent: `hkpShape`) field Info
    /// -   name:`"bvTreeType"`
    /// -   type: `enum BvTreeType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bvTreeType")]
    BvTreeType(Primitive<BvTreeType>),

    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<Unknown>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"child"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "child")]
    Child(HkpSingleShapeContainer),
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppBvTreeShape, "@name",
    ("code" => Code(Primitive<Cow<'de, str>>)),
    ("moppData" => MoppData(Primitive<Cow<'de, str>>)),
    ("moppDataSize" => MoppDataSize(Primitive<u32>)),
    ("codeInfoCopy" => CodeInfoCopy(Vector4<f32>)),
    ("bvTreeType" => BvTreeType(Primitive<BvTreeType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<Unknown>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("child" => Child(HkpSingleShapeContainer)),
    ("childSize" => ChildSize(Primitive<i32>)),
}
