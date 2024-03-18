//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDampingModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbDampingModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x9a040f03`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDampingModifier<'a> {
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
    /// -   name:`"kP"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kP")]
    KP(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"kI"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kI")]
    KI(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"kD"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kD")]
    KD(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enableScalarDamping"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableScalarDamping")]
    EnableScalarDamping(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"enableVectorDamping"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableVectorDamping")]
    EnableVectorDamping(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"rawValue"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawValue")]
    RawValue(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rawVector"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawVector")]
    RawVector(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedVector")]
    DampedVector(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifier<'de>, "@name",
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
    ("kP" => KP(Primitive<f32>)),
    ("kI" => KI(Primitive<f32>)),
    ("kD" => KD(Primitive<f32>)),
    ("enableScalarDamping" => EnableScalarDamping(Primitive<bool>)),
    ("enableVectorDamping" => EnableVectorDamping(Primitive<bool>)),
    ("rawValue" => RawValue(Primitive<f32>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("rawVector" => RawVector(Vector4<f32>)),
    ("dampedVector" => DampedVector(Vector4<f32>)),
    ("vecErrorSum" => VecErrorSum(Vector4<f32>)),
    ("vecPreviousError" => VecPreviousError(Vector4<f32>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}
