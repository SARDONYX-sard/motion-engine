//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbCharacterData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacterData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x300d6808`
/// -   version: 7
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"characterControllerInfo"`
    /// -   type: `struct hkbCharacterDataCharacterControllerInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterControllerInfo")]
    CharacterControllerInfo(HkbCharacterDataCharacterControllerInfo),
    /// # C++ Class Fields Info
    /// -   name:`"modelUpMS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelUpMS")]
    ModelUpMs(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"modelForwardMS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelForwardMS")]
    ModelForwardMs(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"modelRightMS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modelRightMS")]
    ModelRightMs(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray&lt;struct hkbVariableInfo&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"numBonesPerLod"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBonesPerLod")]
    NumBonesPerLod(HkArrayRef<Primitive<i32>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyValues")]
    CharacterPropertyValues(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"footIkDriverInfo"`
    /// -   type: `struct hkbFootIkDriverInfo*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footIkDriverInfo")]
    FootIkDriverInfo(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"handIkDriverInfo"`
    /// -   type: `struct hkbHandIkDriverInfo*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handIkDriverInfo")]
    HandIkDriverInfo(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbCharacterStringData*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSkeletonInfo"`
    /// -   type: `struct hkbMirroredSkeletonInfo*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirroredSkeletonInfo")]
    MirroredSkeletonInfo(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"scale"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scale")]
    Scale(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numHands"`
    /// -   type: `hkInt16`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numHands", skip_serializing)]
    NumHands(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"numFloatSlots"`
    /// -   type: `hkInt16`
    /// - offset: 130
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numFloatSlots", skip_serializing)]
    NumFloatSlots(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterData<'de>, "@name",
    ("characterControllerInfo" => CharacterControllerInfo(HkbCharacterDataCharacterControllerInfo)),
    ("modelUpMS" => ModelUpMs(Vector4<f32>)),
    ("modelForwardMS" => ModelForwardMs(Vector4<f32>)),
    ("modelRightMS" => ModelRightMs(Vector4<f32>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>)),
    ("numBonesPerLod" => NumBonesPerLod(HkArrayRef<Primitive<i32>>)),
    ("characterPropertyValues" => CharacterPropertyValues(Cow<'de, str>)),
    ("footIkDriverInfo" => FootIkDriverInfo(Cow<'de, str>)),
    ("handIkDriverInfo" => HandIkDriverInfo(Cow<'de, str>)),
    ("stringData" => StringData(Cow<'de, str>)),
    ("mirroredSkeletonInfo" => MirroredSkeletonInfo(Cow<'de, str>)),
    ("scale" => Scale(Primitive<f32>)),
    ("numHands" => NumHands(Primitive<i16>)),
    ("numFloatSlots" => NumFloatSlots(Primitive<i16>)),
}
