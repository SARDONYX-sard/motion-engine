//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCachingShapePhantom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCachingShapePhantom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 368
/// -    vtable: true
/// -    parent: `hkpShapePhantom`/`0xcb22fbcd`
/// - signature: `0xcf227f58`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCachingShapePhantom {
    /// # C++ Parent class(`hkpShapePhantom`, parent: `hkpPhantom`) field Info
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionState", default)]
    MotionState(HkMotionState),

    /// # C++ Parent class(`hkpPhantom`, parent: `hkpWorldObject`) field Info
    /// -   name:`"overlapListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "overlapListeners", default, skip_serializing)]
    OverlapListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkpPhantom`, parent: `hkpWorldObject`) field Info
    /// -   name:`"phantomListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomListeners", default, skip_serializing)]
    PhantomListeners(HkArrayRef<Cow<'a, str>>),

    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", default, skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"collidable"`
    /// -   type: `struct hkpLinkedCollidable`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collidable", default)]
    Collidable(HkpLinkedCollidable),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "multiThreadCheck", default)]
    MultiThreadCheck(HkMultiThreadCheck),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"properties"`
    /// -   type: `hkArray&lt;struct hkpProperty&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "properties", default)]
    Properties(HkArrayClass<HkpProperty>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"treeData"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "treeData", default, skip_serializing)]
    TreeData(Primitive<Cow<'a, str>>),

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
    /// -   name:`"collisionDetails"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 352
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionDetails", default, skip_serializing)]
    CollisionDetails(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 364
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "orderDirty", default, skip_serializing)]
    OrderDirty(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCachingShapePhantom, "@name",
    ("motionState" => MotionState(HkMotionState)),
    ("overlapListeners" => OverlapListeners(HkArrayRef<Cow<'de, str>>)),
    ("phantomListeners" => PhantomListeners(HkArrayRef<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(HkpLinkedCollidable)),
    ("multiThreadCheck" => MultiThreadCheck(HkMultiThreadCheck)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("collisionDetails" => CollisionDetails(HkArrayRef<Primitive<()>>)),
    ("orderDirty" => OrderDirty(Primitive<bool>)),
}
