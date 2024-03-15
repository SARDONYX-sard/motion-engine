//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeleton`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaSkeleton`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 84
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x366e8220`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeleton<'a> {
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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"parentIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "parentIndices")]
    ParentIndices(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `hkArray&lt;struct hkaBone&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(HkArrayClass<HkaBone>),
    /// # C++ Class Fields Info
    /// -   name:`"referencePose"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencePose")]
    ReferencePose(HkArrayVector<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"referenceFloats"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceFloats")]
    ReferenceFloats(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"floatSlots"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatSlots")]
    FloatSlots(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"localFrames"`
    /// -   type: `hkArray&lt;struct hkaSkeletonLocalFrameOnBone&gt;`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrames")]
    LocalFrames(HkArrayClass<HkaSkeletonLocalFrameOnBone>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeleton<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parentIndices" => ParentIndices(HkArrayRef<Primitive<i16>>)),
    ("bones" => Bones(HkArrayClass<HkaBone>)),
    ("referencePose" => ReferencePose(HkArrayVector<QsTransform<f32>>)),
    ("referenceFloats" => ReferenceFloats(HkArrayRef<Primitive<f32>>)),
    ("floatSlots" => FloatSlots(HkArrayStringPtr<'de>)),
    ("localFrames" => LocalFrames(HkArrayClass<HkaSkeletonLocalFrameOnBone>)),
}
