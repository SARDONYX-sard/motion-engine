//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbLookAtModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbLookAtModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x3d28e066`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbLookAtModifier<'a> {
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
    /// -   name:`"targetWS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetWS")]
    TargetWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"headForwardLS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "headForwardLS")]
    HeadForwardLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"neckForwardLS"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "neckForwardLS")]
    NeckForwardLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"neckRightLS"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "neckRightLS")]
    NeckRightLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"eyePositionHS"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eyePositionHS")]
    EyePositionHs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"newTargetGain"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "newTargetGain")]
    NewTargetGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleLeft"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleLeft")]
    LimitAngleLeft(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleRight"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleRight")]
    LimitAngleRight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleUp"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleUp")]
    LimitAngleUp(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDown"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDown")]
    LimitAngleDown(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"headIndex"`
    /// -   type: `hkInt16`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "headIndex")]
    HeadIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"neckIndex"`
    /// -   type: `hkInt16`
    /// - offset: 162
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "neckIndex")]
    NeckIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"isOn"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isOn")]
    IsOn(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"individualLimitsOn"`
    /// -   type: `hkBool`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "individualLimitsOn")]
    IndividualLimitsOn(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isTargetInsideLimitCone"`
    /// -   type: `hkBool`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTargetInsideLimitCone")]
    IsTargetInsideLimitCone(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"lookAtLastTargetWS"`
    /// -   type: `hkVector4`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "lookAtLastTargetWS", skip_serializing)]
    LookAtLastTargetWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"lookAtWeight"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "lookAtWeight", skip_serializing)]
    LookAtWeight(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbLookAtModifier<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
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
    ("targetWS" => TargetWs(Primitive<Vector4<f32>>)),
    ("headForwardLS" => HeadForwardLs(Primitive<Vector4<f32>>)),
    ("neckForwardLS" => NeckForwardLs(Primitive<Vector4<f32>>)),
    ("neckRightLS" => NeckRightLs(Primitive<Vector4<f32>>)),
    ("eyePositionHS" => EyePositionHs(Primitive<Vector4<f32>>)),
    ("newTargetGain" => NewTargetGain(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("limitAngleLeft" => LimitAngleLeft(Primitive<f32>)),
    ("limitAngleRight" => LimitAngleRight(Primitive<f32>)),
    ("limitAngleUp" => LimitAngleUp(Primitive<f32>)),
    ("limitAngleDown" => LimitAngleDown(Primitive<f32>)),
    ("headIndex" => HeadIndex(Primitive<i16>)),
    ("neckIndex" => NeckIndex(Primitive<i16>)),
    ("isOn" => IsOn(Primitive<bool>)),
    ("individualLimitsOn" => IndividualLimitsOn(Primitive<bool>)),
    ("isTargetInsideLimitCone" => IsTargetInsideLimitCone(Primitive<bool>)),
    ("lookAtLastTargetWS" => LookAtLastTargetWs(Primitive<Vector4<f32>>)),
    ("lookAtWeight" => LookAtWeight(Primitive<f32>)),
}
