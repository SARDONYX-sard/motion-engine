//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbPoseMatchingGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbPoseMatchingGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbBlenderGenerator`/`0x22df7147`
/// - signature: `0x29e271b4`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoseMatchingGenerator<'a> {
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"referencePoseWeightThreshold"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencePoseWeightThreshold")]
    ReferencePoseWeightThreshold(Primitive<f32>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"blendParameter"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendParameter")]
    BlendParameter(Primitive<f32>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"minCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minCyclicBlendParameter")]
    MinCyclicBlendParameter(Primitive<f32>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"maxCyclicBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCyclicBlendParameter")]
    MaxCyclicBlendParameter(Primitive<f32>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"indexOfSyncMasterChild"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfSyncMasterChild")]
    IndexOfSyncMasterChild(Primitive<i16>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"flags"`
    /// -   type: `hkInt16`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<i16>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"subtractLastChild"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subtractLastChild")]
    SubtractLastChild(Primitive<bool>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkbBlenderGeneratorChild*>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray<void>`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "childrenInternalStates", skip_serializing)]
    ChildrenInternalStates(HkArrayRef<()>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray<void>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "sortedChildren", skip_serializing)]
    SortedChildren(HkArrayRef<()>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "endIntervalWeight", skip_serializing)]
    EndIntervalWeight(Primitive<f32>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "numActiveChildren", skip_serializing)]
    NumActiveChildren(Primitive<i32>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "beginIntervalIndex", skip_serializing)]
    BeginIntervalIndex(Primitive<i16>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 110
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "endIntervalIndex", skip_serializing)]
    EndIntervalIndex(Primitive<i16>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "initSync", skip_serializing)]
    InitSync(Primitive<bool>),
    /// # C++ Parent class(`hkbBlenderGenerator` => parent: `hkbGenerator`) field Info
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "doSubtractiveBlend", skip_serializing)]
    DoSubtractiveBlend(Primitive<bool>),

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
    /// -   name:`"worldFromModelRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelRotation")]
    WorldFromModelRotation(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"blendSpeed"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendSpeed")]
    BlendSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minSpeedToSwitch"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSpeedToSwitch")]
    MinSpeedToSwitch(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minSwitchTimeNoError"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSwitchTimeNoError")]
    MinSwitchTimeNoError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minSwitchTimeFullError"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minSwitchTimeFullError")]
    MinSwitchTimeFullError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startPlayingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startPlayingEventId")]
    StartPlayingEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"startMatchingEventId"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startMatchingEventId")]
    StartMatchingEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootBoneIndex")]
    RootBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "otherBoneIndex")]
    OtherBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anotherBoneIndex")]
    AnotherBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"pelvisIndex"`
    /// -   type: `hkInt16`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pelvisIndex")]
    PelvisIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum Mode`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(Primitive<Mode>),
    /// # C++ Class Fields Info
    /// -   name:`"currentMatch"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "currentMatch", skip_serializing)]
    CurrentMatch(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"bestMatch"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "bestMatch", skip_serializing)]
    BestMatch(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceBetterMatch"`
    /// -   type: `hkReal`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceBetterMatch", skip_serializing)]
    TimeSinceBetterMatch(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "error", skip_serializing)]
    Error(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"resetCurrentMatchLocalTime"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "resetCurrentMatchLocalTime", skip_serializing)]
    ResetCurrentMatchLocalTime(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingUtility"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "poseMatchingUtility", skip_serializing)]
    PoseMatchingUtility(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoseMatchingGenerator<'de>, "@name",
    ("referencePoseWeightThreshold" => ReferencePoseWeightThreshold(Primitive<f32>)),
    ("blendParameter" => BlendParameter(Primitive<f32>)),
    ("minCyclicBlendParameter" => MinCyclicBlendParameter(Primitive<f32>)),
    ("maxCyclicBlendParameter" => MaxCyclicBlendParameter(Primitive<f32>)),
    ("indexOfSyncMasterChild" => IndexOfSyncMasterChild(Primitive<i16>)),
    ("flags" => Flags(Primitive<i16>)),
    ("subtractLastChild" => SubtractLastChild(Primitive<bool>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
    ("childrenInternalStates" => ChildrenInternalStates(HkArrayRef<()>)),
    ("sortedChildren" => SortedChildren(HkArrayRef<()>)),
    ("endIntervalWeight" => EndIntervalWeight(Primitive<f32>)),
    ("numActiveChildren" => NumActiveChildren(Primitive<i32>)),
    ("beginIntervalIndex" => BeginIntervalIndex(Primitive<i16>)),
    ("endIntervalIndex" => EndIntervalIndex(Primitive<i16>)),
    ("initSync" => InitSync(Primitive<bool>)),
    ("doSubtractiveBlend" => DoSubtractiveBlend(Primitive<bool>)),
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
    ("worldFromModelRotation" => WorldFromModelRotation(Primitive<Quaternion<f32>>)),
    ("blendSpeed" => BlendSpeed(Primitive<f32>)),
    ("minSpeedToSwitch" => MinSpeedToSwitch(Primitive<f32>)),
    ("minSwitchTimeNoError" => MinSwitchTimeNoError(Primitive<f32>)),
    ("minSwitchTimeFullError" => MinSwitchTimeFullError(Primitive<f32>)),
    ("startPlayingEventId" => StartPlayingEventId(Primitive<i32>)),
    ("startMatchingEventId" => StartMatchingEventId(Primitive<i32>)),
    ("rootBoneIndex" => RootBoneIndex(Primitive<i16>)),
    ("otherBoneIndex" => OtherBoneIndex(Primitive<i16>)),
    ("anotherBoneIndex" => AnotherBoneIndex(Primitive<i16>)),
    ("pelvisIndex" => PelvisIndex(Primitive<i16>)),
    ("mode" => Mode(Primitive<Mode>)),
    ("currentMatch" => CurrentMatch(Primitive<i32>)),
    ("bestMatch" => BestMatch(Primitive<i32>)),
    ("timeSinceBetterMatch" => TimeSinceBetterMatch(Primitive<f32>)),
    ("error" => Error(Primitive<f32>)),
    ("resetCurrentMatchLocalTime" => ResetCurrentMatchLocalTime(Primitive<bool>)),
    ("poseMatchingUtility" => PoseMatchingUtility(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkbPoseMatchingGenerator<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Mode {
    #[serde(rename = "MODE_MATCH")]
    ModeMatch = 0,
    #[serde(rename = "MODE_PLAY")]
    ModePlay = 1,
}
