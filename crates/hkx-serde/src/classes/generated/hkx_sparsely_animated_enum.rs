//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSparselyAnimatedEnum`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxSparselyAnimatedEnum`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkxSparselyAnimatedInt`/`0xca961951`
/// - signature: `0x68a47b64`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSparselyAnimatedEnum<'a> {
    /// # C++ Parent class(`hkxSparselyAnimatedInt`, parent: `hkReferencedObject`) field Info
    /// -   name:`"ints"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ints")]
    Ints(HkArrayRef<Primitive<i32>>),
    /// # C++ Parent class(`hkxSparselyAnimatedInt`, parent: `hkReferencedObject`) field Info
    /// -   name:`"times"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "times")]
    Times(HkArrayRef<Primitive<f32>>),

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
    /// -   name:`"enum"`
    /// -   type: `struct hkxEnum*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enum")]
    Enum(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedEnum<'de>, "@name",
    ("ints" => Ints(HkArrayRef<Primitive<i32>>)),
    ("times" => Times(HkArrayRef<Primitive<f32>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("enum" => Enum(Primitive<Cow<'de, str>>)),
}
