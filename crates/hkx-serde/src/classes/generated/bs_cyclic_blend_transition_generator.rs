//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSCyclicBlendTransitionGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `BSCyclicBlendTransitionGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x5119eb06`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsCyclicBlendTransitionGenerator<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
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
    CachedBindables(HkArrayRef<()>),
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
    //
    /// # C++ Class Fields Info
    /// -   name:`"pBlenderGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "pBlenderGenerator")]
    PBlenderGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToFreezeBlendValue"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToFreezeBlendValue")]
    EventToFreezeBlendValue(SingleClass<HkbEventProperty<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"EventToCrossBlend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToCrossBlend")]
    EventToCrossBlend(SingleClass<HkbEventProperty<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"fBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fBlendParameter")]
    FBlendParameter(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fTransitionDuration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fTransitionDuration")]
    FTransitionDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"eBlendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eBlendCurve")]
    EBlendCurve(Primitive<BlendCurve>),
    /// # C++ Class Fields Info
    /// -   name:`"pTransitionBlenderGenerator"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|ALIGN16|SERIALIZE_IGNORED`
    #[serde(rename = "pTransitionBlenderGenerator", skip_serializing)]
    PTransitionBlenderGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"pTransitionEffect"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|ALIGN16|SERIALIZE_IGNORED`
    #[serde(rename = "pTransitionEffect", skip_serializing)]
    PTransitionEffect(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"currentMode"`
    /// -   type: `enum unknown`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "currentMode", skip_serializing)]
    CurrentMode(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsCyclicBlendTransitionGenerator<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("pBlenderGenerator" => PBlenderGenerator(Primitive<Cow<'de, str>>)),
    ("EventToFreezeBlendValue" => EventToFreezeBlendValue(SingleClass<HkbEventProperty<'de>>)),
    ("EventToCrossBlend" => EventToCrossBlend(SingleClass<HkbEventProperty<'de>>)),
    ("fBlendParameter" => FBlendParameter(Primitive<f32>)),
    ("fTransitionDuration" => FTransitionDuration(Primitive<f32>)),
    ("eBlendCurve" => EBlendCurve(Primitive<BlendCurve>)),
    ("pTransitionBlenderGenerator" => PTransitionBlenderGenerator(Primitive<Cow<'de, str>>)),
    ("pTransitionEffect" => PTransitionEffect(Primitive<Cow<'de, str>>)),
    ("currentMode" => CurrentMode(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CurrentBlendMode {
    #[serde(rename = "MODE_INACTIVE")]
    ModeInactive = -1,
    #[serde(rename = "MODE_DEFAULT")]
    ModeDefault = 0,
    #[serde(rename = "MODE_FROZEN")]
    ModeFrozen = 1,
    #[serde(rename = "MODE_BLENDING")]
    ModeBlending = 2,
    #[serde(rename = "MODE_WAITINGFORBLENDING")]
    ModeWaitingforblending = 3,
}
