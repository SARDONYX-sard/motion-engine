//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeleton`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeleton<'a> {
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
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"parentIndices"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "parentIndices")]
    ParentIndices(HkArrayNum<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `hkArray<struct hkaBone>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(HkArrayClass<HkaBone<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"referencePose"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencePose")]
    ReferencePose(HkArrayMatrix3<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"referenceFloats"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceFloats")]
    ReferenceFloats(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatSlots"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatSlots")]
    FloatSlots(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"localFrames"`
    /// -   type: `hkArray<struct hkaSkeletonLocalFrameOnBone>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrames")]
    LocalFrames(HkArrayClass<HkaSkeletonLocalFrameOnBone<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeleton<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parentIndices" => ParentIndices(HkArrayNum<i16>)),
    ("bones" => Bones(HkArrayClass<HkaBone<'de>>)),
    ("referencePose" => ReferencePose(HkArrayMatrix3<QsTransform<f32>>)),
    ("referenceFloats" => ReferenceFloats(HkArrayNum<f32>)),
    ("floatSlots" => FloatSlots(HkArrayStringPtr<'de>)),
    ("localFrames" => LocalFrames(HkArrayClass<HkaSkeletonLocalFrameOnBone<'de>>)),
}
