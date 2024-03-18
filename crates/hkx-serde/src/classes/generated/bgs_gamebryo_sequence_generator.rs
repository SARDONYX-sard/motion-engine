//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BGSGamebryoSequenceGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `BGSGamebryoSequenceGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xc8df2d77`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BgsGamebryoSequenceGenerator<'a> {
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
    /// -   name:`"pSequence"`
    /// -   type: `char*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pSequence")]
    PSequence(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eBlendModeFunction"`
    /// -   type: `enum BlendModeFunction`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eBlendModeFunction")]
    EBlendModeFunction(Primitive<BlendModeFunction>),
    /// # C++ Class Fields Info
    /// -   name:`"fPercent"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fPercent")]
    FPercent(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"events"`
    /// -   type: `hkArray<void>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "events", skip_serializing)]
    Events(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"fTime"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "fTime", skip_serializing)]
    FTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bDelayedActivate"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "bDelayedActivate", skip_serializing)]
    BDelayedActivate(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bLooping"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "bLooping", skip_serializing)]
    BLooping(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BgsGamebryoSequenceGenerator<'de>, "@name",
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
    ("pSequence" => PSequence(Primitive<Cow<'de, str>>)),
    ("eBlendModeFunction" => EBlendModeFunction(Primitive<BlendModeFunction>)),
    ("fPercent" => FPercent(Primitive<f32>)),
    ("events" => Events(HkArrayRef<Primitive<()>>)),
    ("fTime" => FTime(Primitive<f32>)),
    ("bDelayedActivate" => BDelayedActivate(Primitive<bool>)),
    ("bLooping" => BLooping(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlendModeFunction {
    #[serde(rename = "BMF_NONE")]
    BmfNone = 0,
    #[serde(rename = "BMF_PERCENT")]
    BmfPercent = 1,
    #[serde(rename = "BMF_ONE_MINUS_PERCENT")]
    BmfOneMinusPercent = 2,
}
