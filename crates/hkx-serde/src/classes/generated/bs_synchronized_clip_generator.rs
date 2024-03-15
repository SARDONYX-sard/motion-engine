//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSSynchronizedClipGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSSynchronizedClipGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xd83bea64`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsSynchronizedClipGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pClipGenerator")]
    PClipGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"SyncAnimPrefix"`
    /// -   type: `char*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "SyncAnimPrefix")]
    SyncAnimPrefix(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"bSyncClipIgnoreMarkPlacement"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bSyncClipIgnoreMarkPlacement")]
    BSyncClipIgnoreMarkPlacement(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"fGetToMarkTime"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fGetToMarkTime")]
    FGetToMarkTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fMarkErrorThreshold"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fMarkErrorThreshold")]
    FMarkErrorThreshold(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bLeadCharacter"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bLeadCharacter")]
    BLeadCharacter(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bReorientSupportChar"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bReorientSupportChar")]
    BReorientSupportChar(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bApplyMotionFromRoot"`
    /// -   type: `hkBool`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bApplyMotionFromRoot")]
    BApplyMotionFromRoot(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"pSyncScene"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSyncScene", skip_serializing)]
    PSyncScene(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"StartMarkWS"`
    /// -   type: `hkQsTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "StartMarkWS", skip_serializing)]
    StartMarkWs(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"EndMarkWS"`
    /// -   type: `hkQsTransform`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "EndMarkWS", skip_serializing)]
    EndMarkWs(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"StartMarkMS"`
    /// -   type: `hkQsTransform`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "StartMarkMS", skip_serializing)]
    StartMarkMs(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fCurrentLerp"`
    /// -   type: `hkReal`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fCurrentLerp", skip_serializing)]
    FCurrentLerp(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pLocalSyncBinding"`
    /// -   type: `void*`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pLocalSyncBinding", skip_serializing)]
    PLocalSyncBinding(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"pEventMap"`
    /// -   type: `void*`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pEventMap", skip_serializing)]
    PEventMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"sAnimationBindingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sAnimationBindingIndex")]
    SAnimationBindingIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"bAtMark"`
    /// -   type: `hkBool`
    /// - offset: 238
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bAtMark", skip_serializing)]
    BAtMark(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bAllCharactersInScene"`
    /// -   type: `hkBool`
    /// - offset: 239
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bAllCharactersInScene", skip_serializing)]
    BAllCharactersInScene(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bAllCharactersAtMarks"`
    /// -   type: `hkBool`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bAllCharactersAtMarks", skip_serializing)]
    BAllCharactersAtMarks(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsSynchronizedClipGenerator<'de>, "@name",
    ("pClipGenerator" => PClipGenerator(Primitive<Cow<'de, str>>)),
    ("SyncAnimPrefix" => SyncAnimPrefix(Primitive<Cow<'de, str>>)),
    ("bSyncClipIgnoreMarkPlacement" => BSyncClipIgnoreMarkPlacement(Primitive<bool>)),
    ("fGetToMarkTime" => FGetToMarkTime(Primitive<f32>)),
    ("fMarkErrorThreshold" => FMarkErrorThreshold(Primitive<f32>)),
    ("bLeadCharacter" => BLeadCharacter(Primitive<bool>)),
    ("bReorientSupportChar" => BReorientSupportChar(Primitive<bool>)),
    ("bApplyMotionFromRoot" => BApplyMotionFromRoot(Primitive<bool>)),
    ("pSyncScene" => PSyncScene(Primitive<Cow<'de, str>>)),
    ("StartMarkWS" => StartMarkWs(QsTransform<f32>)),
    ("EndMarkWS" => EndMarkWs(QsTransform<f32>)),
    ("StartMarkMS" => StartMarkMs(QsTransform<f32>)),
    ("fCurrentLerp" => FCurrentLerp(Primitive<f32>)),
    ("pLocalSyncBinding" => PLocalSyncBinding(Primitive<Cow<'de, str>>)),
    ("pEventMap" => PEventMap(Primitive<Cow<'de, str>>)),
    ("sAnimationBindingIndex" => SAnimationBindingIndex(Primitive<i16>)),
    ("bAtMark" => BAtMark(Primitive<bool>)),
    ("bAllCharactersInScene" => BAllCharactersInScene(Primitive<bool>)),
    ("bAllCharactersAtMarks" => BAllCharactersAtMarks(Primitive<bool>)),
}
