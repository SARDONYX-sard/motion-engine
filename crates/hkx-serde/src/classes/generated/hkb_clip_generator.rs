//! A Rust structure that implements a serializer/deserializer corresponding to `hkbClipGenerator`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 208
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbClipGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbClipGenerator"`: The original C++ class name.
    #[serde(default = "HkbClipGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x333b85b9`: Unique value of this class.
    #[serde(default = "HkbClipGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbClipGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbClipGeneratorHkParam<'a>>
}

impl HkbClipGenerator<'_> {
    /// Return `"hkbClipGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbClipGenerator".into()
    }

    /// Return `"0x333b85b9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x333b85b9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"animationName"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationName")]
    AnimationName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"triggers"`
    /// -   type: `struct hkbClipTriggerArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggers")]
    Triggers(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"cropStartAmountLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cropStartAmountLocalTime")]
    CropStartAmountLocalTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"cropEndAmountLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cropEndAmountLocalTime")]
    CropEndAmountLocalTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"startTime"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startTime")]
    StartTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"playbackSpeed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "playbackSpeed")]
    PlaybackSpeed(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enforcedDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enforcedDuration")]
    EnforcedDuration(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"userControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userControlledTimeFraction")]
    UserControlledTimeFraction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"animationBindingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBindingIndex")]
    AnimationBindingIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"mode"`
    /// -   type: `enum PlaybackMode`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mode")]
    Mode(PlaybackMode),
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `hkInt8`
    /// - offset: 75
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"animDatas"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animDatas", skip_serializing)]
    AnimDatas(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"animationControl"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationControl", skip_serializing)]
    AnimationControl(()),
    /// # Field information in the original C++ class
    /// -   name:`"originalTriggers"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "originalTriggers", skip_serializing)]
    OriginalTriggers(()),
    /// # Field information in the original C++ class
    /// -   name:`"mapperData"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mapperData", skip_serializing)]
    MapperData(()),
    /// # Field information in the original C++ class
    /// -   name:`"binding"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "binding", skip_serializing)]
    Binding(()),
    /// # Field information in the original C++ class
    /// -   name:`"mirroredAnimation"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredAnimation", skip_serializing)]
    MirroredAnimation(()),
    /// # Field information in the original C++ class
    /// -   name:`"extractedMotion"`
    /// -   type: `hkQsTransform`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "extractedMotion", skip_serializing)]
    ExtractedMotion(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"echos"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echos", skip_serializing)]
    Echos(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "localTime", skip_serializing)]
    LocalTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"previousUserControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "previousUserControlledTimeFraction", skip_serializing)]
    PreviousUserControlledTimeFraction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"bufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bufferSize", skip_serializing)]
    BufferSize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"echoBufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "echoBufferSize", skip_serializing)]
    EchoBufferSize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"atEnd"`
    /// -   type: `hkBool`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "atEnd", skip_serializing)]
    AtEnd(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"ignoreStartTime"`
    /// -   type: `hkBool`
    /// - offset: 193
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ignoreStartTime", skip_serializing)]
    IgnoreStartTime(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"pingPongBackward"`
    /// -   type: `hkBool`
    /// - offset: 194
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pingPongBackward", skip_serializing)]
    PingPongBackward(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorHkParam<'de>, "@name",
    ("animationName" => AnimationName(Primitive<Cow<'a, str>>)),
    ("triggers" => Triggers(Cow<'a, str>)),
    ("cropStartAmountLocalTime" => CropStartAmountLocalTime(Primitive<f32>)),
    ("cropEndAmountLocalTime" => CropEndAmountLocalTime(Primitive<f32>)),
    ("startTime" => StartTime(Primitive<f32>)),
    ("playbackSpeed" => PlaybackSpeed(Primitive<f32>)),
    ("enforcedDuration" => EnforcedDuration(Primitive<f32>)),
    ("userControlledTimeFraction" => UserControlledTimeFraction(Primitive<f32>)),
    ("animationBindingIndex" => AnimationBindingIndex(Primitive<i16>)),
    ("mode" => Mode(PlaybackMode)),
    ("flags" => Flags(Primitive<i8>)),
    ("animDatas" => AnimDatas(Vec<()>)),
    ("animationControl" => AnimationControl(())),
    ("originalTriggers" => OriginalTriggers(())),
    ("mapperData" => MapperData(())),
    ("binding" => Binding(())),
    ("mirroredAnimation" => MirroredAnimation(())),
    ("extractedMotion" => ExtractedMotion(QsTransform<f32>)),
    ("echos" => Echos(Vec<()>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("time" => Time(Primitive<f32>)),
    ("previousUserControlledTimeFraction" => PreviousUserControlledTimeFraction(Primitive<f32>)),
    ("bufferSize" => BufferSize(Primitive<i32>)),
    ("echoBufferSize" => EchoBufferSize(Primitive<i32>)),
    ("atEnd" => AtEnd(Primitive<bool>)),
    ("ignoreStartTime" => IgnoreStartTime(Primitive<bool>)),
    ("pingPongBackward" => PingPongBackward(Primitive<bool>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
