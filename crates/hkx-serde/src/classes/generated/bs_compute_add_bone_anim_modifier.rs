//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSComputeAddBoneAnimModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `BSComputeAddBoneAnimModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xa67f8c46`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsComputeAddBoneAnimModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

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
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"translationLSOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationLSOut")]
    TranslationLsOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationLSOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationLSOut")]
    RotationLsOut(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleLSOut"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleLSOut")]
    ScaleLsOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsComputeAddBoneAnimModifier<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<Primitive<()>>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("translationLSOut" => TranslationLsOut(Vector4<f32>)),
    ("rotationLSOut" => RotationLsOut(Quaternion<f32>)),
    ("scaleLSOut" => ScaleLsOut(Vector4<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(Primitive<Cow<'de, str>>)),
}
