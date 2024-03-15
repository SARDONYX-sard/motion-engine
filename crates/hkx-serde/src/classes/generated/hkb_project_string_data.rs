//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProjectStringData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbProjectStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x76ad60a`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProjectStringData<'a> {
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationFilenames")]
    AnimationFilenames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorFilenames")]
    BehaviorFilenames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"characterFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterFilenames")]
    CharacterFilenames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"eventNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventNames")]
    EventNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"animationPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationPath")]
    AnimationPath(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorPath")]
    BehaviorPath(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPath")]
    CharacterPath(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToSource"`
    /// -   type: `hkStringPtr`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fullPathToSource")]
    FullPathToSource(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rootPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rootPath", skip_serializing)]
    RootPath(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProjectStringData<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("animationFilenames" => AnimationFilenames(HkArrayStringPtr<'de>)),
    ("behaviorFilenames" => BehaviorFilenames(HkArrayStringPtr<'de>)),
    ("characterFilenames" => CharacterFilenames(HkArrayStringPtr<'de>)),
    ("eventNames" => EventNames(HkArrayStringPtr<'de>)),
    ("animationPath" => AnimationPath(Primitive<Cow<'de, str>>)),
    ("behaviorPath" => BehaviorPath(Primitive<Cow<'de, str>>)),
    ("characterPath" => CharacterPath(Primitive<Cow<'de, str>>)),
    ("fullPathToSource" => FullPathToSource(Primitive<Cow<'de, str>>)),
    ("rootPath" => RootPath(Primitive<Cow<'de, str>>)),
}
