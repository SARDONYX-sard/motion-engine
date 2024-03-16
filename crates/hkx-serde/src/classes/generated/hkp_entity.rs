//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpEntity`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpEntity`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 544
/// -    vtable: true
/// -    parent: `hkpWorldObject`/`0x49fb6f2e`
/// - signature: `0xa03c774b`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpEntity<'a> {
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"collidable"`
    /// -   type: `struct hkpLinkedCollidable`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collidable")]
    Collidable(HkpLinkedCollidable<'a>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "multiThreadCheck")]
    MultiThreadCheck(HkMultiThreadCheck),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"properties"`
    /// -   type: `hkArray&lt;struct hkpProperty&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "properties")]
    Properties(HkArrayClass<HkpProperty>),
    /// # C++ Parent class(`hkpWorldObject`, parent: `hkReferencedObject`) field Info
    /// -   name:`"treeData"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "treeData", skip_serializing)]
    TreeData(Primitive<Cow<'a, str>>),

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
    /// -   name:`"material"`
    /// -   type: `struct hkpMaterial`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(HkpMaterial),
    /// # C++ Class Fields Info
    /// -   name:`"limitContactImpulseUtilAndFlag"`
    /// -   type: `void*`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "limitContactImpulseUtilAndFlag", skip_serializing)]
    LimitContactImpulseUtilAndFlag(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damageMultiplier")]
    DamageMultiplier(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"breakableBody"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "breakableBody", skip_serializing)]
    BreakableBody(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"solverData"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "solverData", skip_serializing)]
    SolverData(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"storageIndex"`
    /// -   type: `hkUint16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "storageIndex")]
    StorageIndex(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"contactPointCallbackDelay"`
    /// -   type: `hkUint16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointCallbackDelay")]
    ContactPointCallbackDelay(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintsMaster"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "constraintsMaster", skip_serializing)]
    ConstraintsMaster(HkpEntitySmallArraySerializeOverrideType<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintsSlave"`
    /// -   type: `hkArray&lt;hkpConstraintInstance*&gt;`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE | NOT_OWNED | SERIALIZE_IGNORED`
    #[serde(rename = "constraintsSlave", skip_serializing)]
    ConstraintsSlave(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintRuntime"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "constraintRuntime", skip_serializing)]
    ConstraintRuntime(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"simulationIsland"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "simulationIsland", skip_serializing)]
    SimulationIsland(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"autoRemoveLevel"`
    /// -   type: `hkInt8`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoRemoveLevel")]
    AutoRemoveLevel(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"numShapeKeysInContactPointProperties"`
    /// -   type: `hkUint8`
    /// - offset: 209
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numShapeKeysInContactPointProperties")]
    NumShapeKeysInContactPointProperties(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"responseModifierFlags"`
    /// -   type: `hkUint8`
    /// - offset: 210
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "responseModifierFlags")]
    ResponseModifierFlags(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uid")]
    Uid(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"spuCollisionCallback"`
    /// -   type: `struct hkpEntitySpuCollisionCallback`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spuCollisionCallback")]
    SpuCollisionCallback(HkpEntitySpuCollisionCallback<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"motion"`
    /// -   type: `struct hkpMaxSizeMotion`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motion")]
    Motion(HkpMaxSizeMotion<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"contactListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 512
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactListeners", skip_serializing)]
    ContactListeners(HkpEntitySmallArraySerializeOverrideType<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"actions"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 520
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "actions", skip_serializing)]
    Actions(HkpEntitySmallArraySerializeOverrideType<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"localFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 528
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrame")]
    LocalFrame(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"extendedListeners"`
    /// -   type: `struct hkpEntityExtendedListeners*`
    /// - offset: 532
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "extendedListeners", skip_serializing)]
    ExtendedListeners(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"npData"`
    /// -   type: `hkUint32`
    /// - offset: 536
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "npData")]
    NpData(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntity<'de>, "@name",
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("collidable" => Collidable(HkpLinkedCollidable<'de>)),
    ("multiThreadCheck" => MultiThreadCheck(HkMultiThreadCheck)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkpProperty>)),
    ("treeData" => TreeData(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("material" => Material(HkpMaterial)),
    ("limitContactImpulseUtilAndFlag" => LimitContactImpulseUtilAndFlag(Primitive<Cow<'de, str>>)),
    ("damageMultiplier" => DamageMultiplier(Primitive<f32>)),
    ("breakableBody" => BreakableBody(Primitive<Cow<'de, str>>)),
    ("solverData" => SolverData(Primitive<u32>)),
    ("storageIndex" => StorageIndex(Primitive<u16>)),
    ("contactPointCallbackDelay" => ContactPointCallbackDelay(Primitive<u16>)),
    ("constraintsMaster" => ConstraintsMaster(HkpEntitySmallArraySerializeOverrideType<'de>)),
    ("constraintsSlave" => ConstraintsSlave(HkArrayRef<Cow<'de, str>>)),
    ("constraintRuntime" => ConstraintRuntime(HkArrayRef<Primitive<u8>>)),
    ("simulationIsland" => SimulationIsland(Primitive<Cow<'de, str>>)),
    ("autoRemoveLevel" => AutoRemoveLevel(Primitive<i8>)),
    ("numShapeKeysInContactPointProperties" => NumShapeKeysInContactPointProperties(Primitive<u8>)),
    ("responseModifierFlags" => ResponseModifierFlags(Primitive<u8>)),
    ("uid" => Uid(Primitive<u32>)),
    ("spuCollisionCallback" => SpuCollisionCallback(HkpEntitySpuCollisionCallback<'de>)),
    ("motion" => Motion(HkpMaxSizeMotion<'de>)),
    ("contactListeners" => ContactListeners(HkpEntitySmallArraySerializeOverrideType<'de>)),
    ("actions" => Actions(HkpEntitySmallArraySerializeOverrideType<'de>)),
    ("localFrame" => LocalFrame(Primitive<Cow<'de, str>>)),
    ("extendedListeners" => ExtendedListeners(Primitive<Cow<'de, str>>)),
    ("npData" => NpData(Primitive<u32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpuCollisionCallbackEventFilter {
    #[serde(rename = "SPU_SEND_NONE")]
    SpuSendNone = 0,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_ADDED")]
    SpuSendContactPointAdded = 1,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_PROCESS")]
    SpuSendContactPointProcess = 2,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_REMOVED")]
    SpuSendContactPointRemoved = 4,
    #[serde(rename = "SPU_SEND_CONTACT_POINT_ADDED_OR_PROCESS")]
    SpuSendContactPointAddedOrProcess = 3,
}
