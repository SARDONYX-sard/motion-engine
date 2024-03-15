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
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"nearbyCharacters"`
    /// -   type: `hkArray&lt;hkbCharacter*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nearbyCharacters", default)]
    NearbyCharacters(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"currentLod"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentLod", default)]
    CurrentLod(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"numTracksInLod"`
    /// -   type: `hkInt16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numTracksInLod", default, skip_serializing)]
    NumTracksInLod(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollDriver"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ragdollDriver", default, skip_serializing)]
    RagdollDriver(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterControllerDriver"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterControllerDriver", default, skip_serializing)]
    CharacterControllerDriver(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"footIkDriver"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "footIkDriver", default, skip_serializing)]
    FootIkDriver(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"handIkDriver"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "handIkDriver", default, skip_serializing)]
    HandIkDriver(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"setup"`
    /// -   type: `struct hkbCharacterSetup*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setup", default)]
    Setup(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorGraph"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorGraph", default)]
    BehaviorGraph(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"projectData"`
    /// -   type: `struct hkbProjectData*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "projectData", default)]
    ProjectData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationBindingSet", default, skip_serializing)]
    AnimationBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"raycastInterface"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "raycastInterface", default, skip_serializing)]
    RaycastInterface(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | NOT_OWNED | SERIALIZE_IGNORED`
    #[serde(rename = "world", default, skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventQueue", default, skip_serializing)]
    EventQueue(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModel"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldFromModel", default, skip_serializing)]
    WorldFromModel(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"poseLocal"`
    /// -   type: `hkSimpleArray&lt;void&gt;`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "poseLocal", default, skip_serializing)]
    PoseLocal(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"deleteWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "deleteWorldFromModel", default, skip_serializing)]
    DeleteWorldFromModel(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"deletePoseLocal"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "deletePoseLocal", default, skip_serializing)]
    DeletePoseLocal(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacter<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("nearbyCharacters" => NearbyCharacters(HkArrayRef<Cow<'de, str>>)),
    ("currentLod" => CurrentLod(Primitive<i16>)),
    ("numTracksInLod" => NumTracksInLod(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("ragdollDriver" => RagdollDriver(Primitive<Cow<'de, str>>)),
    ("characterControllerDriver" => CharacterControllerDriver(Primitive<Cow<'de, str>>)),
    ("footIkDriver" => FootIkDriver(Primitive<Cow<'de, str>>)),
    ("handIkDriver" => HandIkDriver(Primitive<Cow<'de, str>>)),
    ("setup" => Setup(Primitive<Cow<'de, str>>)),
    ("behaviorGraph" => BehaviorGraph(Primitive<Cow<'de, str>>)),
    ("projectData" => ProjectData(Primitive<Cow<'de, str>>)),
    ("animationBindingSet" => AnimationBindingSet(Primitive<Cow<'de, str>>)),
    ("raycastInterface" => RaycastInterface(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(Primitive<Cow<'de, str>>)),
    ("worldFromModel" => WorldFromModel(Primitive<Cow<'de, str>>)),
    ("poseLocal" => PoseLocal(HkArrayRef<Primitive<()>>)),
    ("deleteWorldFromModel" => DeleteWorldFromModel(Primitive<bool>)),
    ("deletePoseLocal" => DeletePoseLocal(Primitive<bool>)),
}
