//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEvaluateHandleModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbEvaluateHandleModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x79757102`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateHandleModifier<'a> {
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
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handle")]
    Handle(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"handlePositionOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handlePositionOut")]
    HandlePositionOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleRotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleRotationOut")]
    HandleRotationOut(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isValidOut"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isValidOut")]
    IsValidOut(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeSpeed"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeSpeed")]
    HandleChangeSpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"handleChangeMode"`
    /// -   type: `enum HandleChangeMode`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleChangeMode")]
    HandleChangeMode(Primitive<HandleChangeMode>),
    /// # C++ Class Fields Info
    /// -   name:`"oldHandle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "oldHandle", skip_serializing)]
    OldHandle(SingleClass<HkbHandle<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"oldHandlePosition"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "oldHandlePosition", skip_serializing)]
    OldHandlePosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"oldHandleRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "oldHandleRotation", skip_serializing)]
    OldHandleRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"smoothlyChangingHandles"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "smoothlyChangingHandles", skip_serializing)]
    SmoothlyChangingHandles(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateHandleModifier<'de>, "@name",
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
    ("handle" => Handle(Primitive<Cow<'de, str>>)),
    ("handlePositionOut" => HandlePositionOut(Vector4<f32>)),
    ("handleRotationOut" => HandleRotationOut(Quaternion<f32>)),
    ("isValidOut" => IsValidOut(Primitive<bool>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("handleChangeSpeed" => HandleChangeSpeed(Primitive<f32>)),
    ("handleChangeMode" => HandleChangeMode(Primitive<HandleChangeMode>)),
    ("oldHandle" => OldHandle(SingleClass<HkbHandle<'de>>)),
    ("oldHandlePosition" => OldHandlePosition(Vector4<f32>)),
    ("oldHandleRotation" => OldHandleRotation(Quaternion<f32>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
    ("smoothlyChangingHandles" => SmoothlyChangingHandles(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandleChangeMode {
    #[serde(rename = "HANDLE_CHANGE_MODE_ABRUPT")]
    HandleChangeModeAbrupt = 0,
    #[serde(rename = "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY")]
    HandleChangeModeConstantVelocity = 1,
}
