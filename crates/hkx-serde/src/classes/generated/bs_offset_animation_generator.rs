//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSOffsetAnimationGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `BSOffsetAnimationGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xb8571122`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsOffsetAnimationGenerator<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields

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
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"pOffsetClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "pOffsetClipGenerator")]
    POffsetClipGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetVariable"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetVariable")]
    FOffsetVariable(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetRangeStart"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetRangeStart")]
    FOffsetRangeStart(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fOffsetRangeEnd"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetRangeEnd")]
    FOffsetRangeEnd(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"BoneOffsetA"`
    /// -   type: `hkArray<void>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "BoneOffsetA", skip_serializing)]
    BoneOffsetA(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"BoneIndexA"`
    /// -   type: `hkArray<void>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "BoneIndexA", skip_serializing)]
    BoneIndexA(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"fCurrentPercentage"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "fCurrentPercentage", skip_serializing)]
    FCurrentPercentage(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"iCurrentFrame"`
    /// -   type: `hkUint32`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "iCurrentFrame", skip_serializing)]
    ICurrentFrame(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"bZeroOffset"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "bZeroOffset", skip_serializing)]
    BZeroOffset(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bOffsetValid"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "bOffsetValid", skip_serializing)]
    BOffsetValid(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsOffsetAnimationGenerator<'de>, "@name",
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
    ("pDefaultGenerator" => PDefaultGenerator(Primitive<Cow<'de, str>>)),
    ("pOffsetClipGenerator" => POffsetClipGenerator(Primitive<Cow<'de, str>>)),
    ("fOffsetVariable" => FOffsetVariable(Primitive<f32>)),
    ("fOffsetRangeStart" => FOffsetRangeStart(Primitive<f32>)),
    ("fOffsetRangeEnd" => FOffsetRangeEnd(Primitive<f32>)),
    ("BoneOffsetA" => BoneOffsetA(HkArrayRef<Primitive<()>>)),
    ("BoneIndexA" => BoneIndexA(HkArrayRef<Primitive<()>>)),
    ("fCurrentPercentage" => FCurrentPercentage(Primitive<f32>)),
    ("iCurrentFrame" => ICurrentFrame(Primitive<u32>)),
    ("bZeroOffset" => BZeroOffset(Primitive<bool>)),
    ("bOffsetValid" => BOffsetValid(Primitive<bool>)),
}
