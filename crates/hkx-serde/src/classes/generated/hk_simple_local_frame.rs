//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkSimpleLocalFrame`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkSimpleLocalFrame`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkLocalFrame`/`0xda8c7d7d`
/// - signature: `0xe758f63c`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSimpleLocalFrame<'a> {
    // `hkLocalFrame`(Parent class) has no fields

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
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform", default)]
    Transform(Transform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkLocalFrame*&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children", default)]
    Children(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"parentFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | NOT_OWNED`
    #[serde(rename = "parentFrame", default)]
    ParentFrame(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"group"`
    /// -   type: `struct hkLocalFrameGroup*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "group", default)]
    Group(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkSimpleLocalFrame<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("transform" => Transform(Transform<f32>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
    ("parentFrame" => ParentFrame(Primitive<Cow<'de, str>>)),
    ("group" => Group(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
