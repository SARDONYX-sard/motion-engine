//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWorldObject`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpWorldObject`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 140
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x49fb6f2e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWorldObject<'a> {
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
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"collidable"`
    /// -   type: `struct hkpLinkedCollidable`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collidable")]
    Collidable(HkpLinkedCollidable),
    /// # C++ Class Fields Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "multiThreadCheck")]
    MultiThreadCheck(HkMultiThreadCheck),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"properties"`
    /// -   type: `hkArray&lt;struct hkpProperty&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "properties")]
    Properties(HkArrayClass<HkpProperty>),
    /// # C++ Class Fields Info
    /// -   name:`"treeData"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "treeData", skip_serializing)]
    TreeData(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldObject<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(HkpLinkedCollidable)),
    ("multiThreadCheck" => MultiThreadCheck(HkMultiThreadCheck)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MtChecks {
    #[serde(rename = "MULTI_THREADING_CHECKS_ENABLE")]
    MultiThreadingChecksEnable = 0,
    #[serde(rename = "MULTI_THREADING_CHECKS_IGNORE")]
    MultiThreadingChecksIgnore = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BroadPhaseType {
    #[serde(rename = "BROAD_PHASE_INVALID")]
    BroadPhaseInvalid = 0,
    #[serde(rename = "BROAD_PHASE_ENTITY")]
    BroadPhaseEntity = 1,
    #[serde(rename = "BROAD_PHASE_PHANTOM")]
    BroadPhasePhantom = 2,
    #[serde(rename = "BROAD_PHASE_BORDER")]
    BroadPhaseBorder = 3,
    #[serde(rename = "BROAD_PHASE_MAX_ID")]
    BroadPhaseMaxId = 4,
}
