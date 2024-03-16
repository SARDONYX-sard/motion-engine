//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterSetup`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacterSetup`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xe5a2a413`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterSetup<'a> {
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
    /// -   name:`"retargetingSkeletonMappers"`
    /// -   type: `hkArray&lt;hkaSkeletonMapper*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "retargetingSkeletonMappers")]
    RetargetingSkeletonMappers(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationSkeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationSkeleton")]
    AnimationSkeleton(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollToAnimationSkeletonMapper"`
    /// -   type: `struct hkaSkeletonMapper*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollToAnimationSkeletonMapper")]
    RagdollToAnimationSkeletonMapper(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationToRagdollSkeletonMapper"`
    /// -   type: `struct hkaSkeletonMapper*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationToRagdollSkeletonMapper")]
    AnimationToRagdollSkeletonMapper(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationBindingSet", skip_serializing)]
    AnimationBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkbCharacterData*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSkeleton"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredSkeleton", skip_serializing)]
    MirroredSkeleton(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPropertyIdMap", skip_serializing)]
    CharacterPropertyIdMap(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterSetup<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("retargetingSkeletonMappers" => RetargetingSkeletonMappers(HkArrayRef<Cow<'de, str>>)),
    ("animationSkeleton" => AnimationSkeleton(Primitive<Cow<'de, str>>)),
    ("ragdollToAnimationSkeletonMapper" => RagdollToAnimationSkeletonMapper(Primitive<Cow<'de, str>>)),
    ("animationToRagdollSkeletonMapper" => AnimationToRagdollSkeletonMapper(Primitive<Cow<'de, str>>)),
    ("animationBindingSet" => AnimationBindingSet(Primitive<Cow<'de, str>>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("mirroredSkeleton" => MirroredSkeleton(Primitive<Cow<'de, str>>)),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(Primitive<Cow<'de, str>>)),
}
