//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGetUpModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbGetUpModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x61cb7ac0`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGetUpModifier<'a> {
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
    /// -   name:`"groundNormal"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundNormal")]
    GroundNormal(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"alignWithGroundDuration"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundDuration")]
    AlignWithGroundDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootBoneIndex")]
    RootBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "otherBoneIndex")]
    OtherBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anotherBoneIndex")]
    AnotherBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceBegin"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceBegin", skip_serializing)]
    TimeSinceBegin(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"initNextModify"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "initNextModify", skip_serializing)]
    InitNextModify(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetUpModifier<'de>, "@name",
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
    ("groundNormal" => GroundNormal(Primitive<Vector4<f32>>)),
    ("duration" => Duration(Primitive<f32>)),
    ("alignWithGroundDuration" => AlignWithGroundDuration(Primitive<f32>)),
    ("rootBoneIndex" => RootBoneIndex(Primitive<i16>)),
    ("otherBoneIndex" => OtherBoneIndex(Primitive<i16>)),
    ("anotherBoneIndex" => AnotherBoneIndex(Primitive<i16>)),
    ("timeSinceBegin" => TimeSinceBegin(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("initNextModify" => InitNextModify(Primitive<bool>)),
}

impl ByteDeSerialize for HkbGetUpModifier<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
