//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbClipGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x333b85b9`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipGenerator<'a> {
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
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<bool, 1>),

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

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
    /// -   name:`"animationName"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationName")]
    AnimationName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"triggers"`
    /// -   type: `struct hkbClipTriggerArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggers")]
    Triggers(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"cropStartAmountLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cropStartAmountLocalTime")]
    CropStartAmountLocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cropEndAmountLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cropEndAmountLocalTime")]
    CropEndAmountLocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"startTime"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startTime")]
    StartTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"playbackSpeed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "playbackSpeed")]
    PlaybackSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enforcedDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforcedDuration")]
    EnforcedDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"userControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userControlledTimeFraction")]
    UserControlledTimeFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBindingIndex")]
    AnimationBindingIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum PlaybackMode`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(Primitive<PlaybackMode>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkInt8`
    /// - offset: 75
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"animDatas"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animDatas", skip_serializing)]
    AnimDatas(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationControl"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationControl", skip_serializing)]
    AnimationControl(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"originalTriggers"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "originalTriggers", skip_serializing)]
    OriginalTriggers(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"mapperData"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mapperData", skip_serializing)]
    MapperData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"binding"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "binding", skip_serializing)]
    Binding(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredAnimation"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredAnimation", skip_serializing)]
    MirroredAnimation(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"extractedMotion"`
    /// -   type: `hkQsTransform`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "extractedMotion", skip_serializing)]
    ExtractedMotion(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"echos"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echos", skip_serializing)]
    Echos(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "localTime", skip_serializing)]
    LocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousUserControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "previousUserControlledTimeFraction", skip_serializing)]
    PreviousUserControlledTimeFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bufferSize", skip_serializing)]
    BufferSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"echoBufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoBufferSize", skip_serializing)]
    EchoBufferSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"atEnd"`
    /// -   type: `hkBool`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "atEnd", skip_serializing)]
    AtEnd(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"ignoreStartTime"`
    /// -   type: `hkBool`
    /// - offset: 193
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ignoreStartTime", skip_serializing)]
    IgnoreStartTime(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"pingPongBackward"`
    /// -   type: `hkBool`
    /// - offset: 194
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pingPongBackward", skip_serializing)]
    PingPongBackward(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGenerator<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<Unknown>)),
    ("padNode" => PadNode(CStyleArray<bool, 1>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<Primitive<()>>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("animationName" => AnimationName(Primitive<Cow<'de, str>>)),
    ("triggers" => Triggers(Primitive<Cow<'de, str>>)),
    ("cropStartAmountLocalTime" => CropStartAmountLocalTime(Primitive<f32>)),
    ("cropEndAmountLocalTime" => CropEndAmountLocalTime(Primitive<f32>)),
    ("startTime" => StartTime(Primitive<f32>)),
    ("playbackSpeed" => PlaybackSpeed(Primitive<f32>)),
    ("enforcedDuration" => EnforcedDuration(Primitive<f32>)),
    ("userControlledTimeFraction" => UserControlledTimeFraction(Primitive<f32>)),
    ("animationBindingIndex" => AnimationBindingIndex(Primitive<i16>)),
    ("mode" => Mode(Primitive<PlaybackMode>)),
    ("flags" => Flags(Primitive<i8>)),
    ("animDatas" => AnimDatas(HkArrayRef<Primitive<()>>)),
    ("animationControl" => AnimationControl(Primitive<Cow<'de, str>>)),
    ("originalTriggers" => OriginalTriggers(Primitive<Cow<'de, str>>)),
    ("mapperData" => MapperData(Primitive<Cow<'de, str>>)),
    ("binding" => Binding(Primitive<Cow<'de, str>>)),
    ("mirroredAnimation" => MirroredAnimation(Primitive<Cow<'de, str>>)),
    ("extractedMotion" => ExtractedMotion(QsTransform<f32>)),
    ("echos" => Echos(HkArrayRef<Primitive<()>>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("time" => Time(Primitive<f32>)),
    ("previousUserControlledTimeFraction" => PreviousUserControlledTimeFraction(Primitive<f32>)),
    ("bufferSize" => BufferSize(Primitive<i32>)),
    ("echoBufferSize" => EchoBufferSize(Primitive<i32>)),
    ("atEnd" => AtEnd(Primitive<bool>)),
    ("ignoreStartTime" => IgnoreStartTime(Primitive<bool>)),
    ("pingPongBackward" => PingPongBackward(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaybackMode {
    #[serde(rename = "MODE_SINGLE_PLAY")]
    ModeSinglePlay = 0,
    #[serde(rename = "MODE_LOOPING")]
    ModeLooping = 1,
    #[serde(rename = "MODE_USER_CONTROLLED")]
    ModeUserControlled = 2,
    #[serde(rename = "MODE_PING_PONG")]
    ModePingPong = 3,
    #[serde(rename = "MODE_COUNT")]
    ModeCount = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClipFlags {
    #[serde(rename = "FLAG_CONTINUE_MOTION_AT_END")]
    FlagContinueMotionAtEnd = 1,
    #[serde(rename = "FLAG_SYNC_HALF_CYCLE_IN_PING_PONG_MODE")]
    FlagSyncHalfCycleInPingPongMode = 2,
    #[serde(rename = "FLAG_MIRROR")]
    FlagMirror = 4,
    #[serde(rename = "FLAG_FORCE_DENSE_POSE")]
    FlagForceDensePose = 8,
    #[serde(rename = "FLAG_DONT_CONVERT_ANNOTATIONS_TO_TRIGGERS")]
    FlagDontConvertAnnotationsToTriggers = 16,
    #[serde(rename = "FLAG_IGNORE_MOTION")]
    FlagIgnoreMotion = 32,
}
