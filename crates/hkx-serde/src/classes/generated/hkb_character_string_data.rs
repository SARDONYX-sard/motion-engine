//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterStringData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbCharacterStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 132
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x655b42bc`
/// -   version: 5
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterStringData<'a> {
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

    /// # C++ Class Fields Info
    /// -   name:`"deformableSkinNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deformableSkinNames")]
    DeformableSkinNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"rigidSkinNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidSkinNames")]
    RigidSkinNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"animationNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationNames")]
    AnimationNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationFilenames")]
    AnimationFilenames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyNames")]
    CharacterPropertyNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"retargetingSkeletonMapperFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "retargetingSkeletonMapperFilenames")]
    RetargetingSkeletonMapperFilenames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"lodNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lodNames")]
    LodNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSyncPointSubstringsA"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSyncPointSubstringsA")]
    MirroredSyncPointSubstringsA(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSyncPointSubstringsB"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSyncPointSubstringsB")]
    MirroredSyncPointSubstringsB(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rigName"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigName")]
    RigName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollName"`
    /// -   type: `hkStringPtr`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollName")]
    RagdollName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorFilename"`
    /// -   type: `hkStringPtr`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorFilename")]
    BehaviorFilename(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterStringData<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("deformableSkinNames" => DeformableSkinNames(HkArrayStringPtr<'de>)),
    ("rigidSkinNames" => RigidSkinNames(HkArrayStringPtr<'de>)),
    ("animationNames" => AnimationNames(HkArrayStringPtr<'de>)),
    ("animationFilenames" => AnimationFilenames(HkArrayStringPtr<'de>)),
    ("characterPropertyNames" => CharacterPropertyNames(HkArrayStringPtr<'de>)),
    ("retargetingSkeletonMapperFilenames" => RetargetingSkeletonMapperFilenames(HkArrayStringPtr<'de>)),
    ("lodNames" => LodNames(HkArrayStringPtr<'de>)),
    ("mirroredSyncPointSubstringsA" => MirroredSyncPointSubstringsA(HkArrayStringPtr<'de>)),
    ("mirroredSyncPointSubstringsB" => MirroredSyncPointSubstringsB(HkArrayStringPtr<'de>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("rigName" => RigName(Primitive<Cow<'de, str>>)),
    ("ragdollName" => RagdollName(Primitive<Cow<'de, str>>)),
    ("behaviorFilename" => BehaviorFilename(Primitive<Cow<'de, str>>)),
}
