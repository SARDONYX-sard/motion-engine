//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3088a5c5`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacter<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"nearbyCharacters"`
    /// -   type: `hkArray&lt;hkbCharacter*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nearbyCharacters")]
    NearbyCharacters(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"currentLod"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentLod")]
    CurrentLod(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"numTracksInLod"`
    /// -   type: `hkInt16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numTracksInLod", skip_serializing)]
    NumTracksInLod(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollDriver"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ragdollDriver", skip_serializing)]
    RagdollDriver(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"characterControllerDriver"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterControllerDriver", skip_serializing)]
    CharacterControllerDriver(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"footIkDriver"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "footIkDriver", skip_serializing)]
    FootIkDriver(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"handIkDriver"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "handIkDriver", skip_serializing)]
    HandIkDriver(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"setup"`
    /// -   type: `struct hkbCharacterSetup*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setup")]
    Setup(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorGraph"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorGraph")]
    BehaviorGraph(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"projectData"`
    /// -   type: `struct hkbProjectData*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "projectData")]
    ProjectData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationBindingSet", skip_serializing)]
    AnimationBindingSet(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"raycastInterface"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "raycastInterface", skip_serializing)]
    RaycastInterface(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | NOT_OWNED | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventQueue", skip_serializing)]
    EventQueue(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModel"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldFromModel", skip_serializing)]
    WorldFromModel(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"poseLocal"`
    /// -   type: `hkSimpleArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "poseLocal", skip_serializing)]
    PoseLocal(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"deleteWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "deleteWorldFromModel", skip_serializing)]
    DeleteWorldFromModel(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"deletePoseLocal"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "deletePoseLocal", skip_serializing)]
    DeletePoseLocal(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacter<'de>, "@name",
    ("nearbyCharacters" => NearbyCharacters(HkArrayRef<Cow<'de, str>>)),
    ("currentLod" => CurrentLod(Primitive<i16>)),
    ("numTracksInLod" => NumTracksInLod(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("ragdollDriver" => RagdollDriver(Cow<'de, str>)),
    ("characterControllerDriver" => CharacterControllerDriver(Cow<'de, str>)),
    ("footIkDriver" => FootIkDriver(Cow<'de, str>)),
    ("handIkDriver" => HandIkDriver(Cow<'de, str>)),
    ("setup" => Setup(Cow<'de, str>)),
    ("behaviorGraph" => BehaviorGraph(Cow<'de, str>)),
    ("projectData" => ProjectData(Cow<'de, str>)),
    ("animationBindingSet" => AnimationBindingSet(Cow<'de, str>)),
    ("raycastInterface" => RaycastInterface(Cow<'de, str>)),
    ("world" => World(Cow<'de, str>)),
    ("eventQueue" => EventQueue(Cow<'de, str>)),
    ("worldFromModel" => WorldFromModel(Cow<'de, str>)),
    ("poseLocal" => PoseLocal(HkArrayRef<()>)),
    ("deleteWorldFromModel" => DeleteWorldFromModel(Primitive<bool>)),
    ("deletePoseLocal" => DeletePoseLocal(Primitive<bool>)),
}
