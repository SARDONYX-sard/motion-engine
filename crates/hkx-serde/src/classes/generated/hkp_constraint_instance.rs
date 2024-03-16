//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintInstance`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x34eba5f`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintInstance<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"owner"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "owner", skip_serializing)]
    Owner(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintModifiers"`
    /// -   type: `struct hkpModifierConstraintAtom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintModifiers")]
    ConstraintModifiers(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"entities"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entities")]
    Entities(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"priority"`
    /// -   type: `enum ConstraintPriority`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "priority")]
    Priority(Primitive<ConstraintPriority>),
    /// # C++ Class Fields Info
    /// -   name:`"wantRuntime"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantRuntime")]
    WantRuntime(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"destructionRemapInfo"`
    /// -   type: `enum OnDestructionRemapInfo`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "destructionRemapInfo")]
    DestructionRemapInfo(Primitive<OnDestructionRemapInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"listeners"`
    /// -   type: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"internal"`
    /// -   type: `void*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internal", skip_serializing)]
    Internal(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "uid", skip_serializing)]
    Uid(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintInstance<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("owner" => Owner(Primitive<Cow<'de, str>>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("constraintModifiers" => ConstraintModifiers(Primitive<Cow<'de, str>>)),
    ("entities" => Entities(Primitive<Cow<'de, str>>)),
    ("priority" => Priority(Primitive<ConstraintPriority>)),
    ("wantRuntime" => WantRuntime(Primitive<bool>)),
    ("destructionRemapInfo" => DestructionRemapInfo(Primitive<OnDestructionRemapInfo>)),
    ("listeners" => Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType<'de>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("internal" => Internal(Primitive<Cow<'de, str>>)),
    ("uid" => Uid(Primitive<u32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintPriority {
    #[serde(rename = "PRIORITY_INVALID")]
    PriorityInvalid = 0,
    #[serde(rename = "PRIORITY_PSI")]
    PriorityPsi = 1,
    #[serde(rename = "PRIORITY_SIMPLIFIED_TOI_UNUSED")]
    PrioritySimplifiedToiUnused = 2,
    #[serde(rename = "PRIORITY_TOI")]
    PriorityToi = 3,
    #[serde(rename = "PRIORITY_TOI_HIGHER")]
    PriorityToiHigher = 4,
    #[serde(rename = "PRIORITY_TOI_FORCED")]
    PriorityToiForced = 5,
    #[serde(rename = "NUM_PRIORITIES")]
    NumPriorities = 6,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstanceType {
    #[serde(rename = "TYPE_NORMAL")]
    TypeNormal = 0,
    #[serde(rename = "TYPE_CHAIN")]
    TypeChain = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddReferences {
    #[serde(rename = "DO_NOT_ADD_REFERENCES")]
    DoNotAddReferences = 0,
    #[serde(rename = "DO_ADD_REFERENCES")]
    DoAddReferences = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloningMode {
    #[serde(rename = "CLONE_SHALLOW_IF_NOT_CONSTRAINED_TO_WORLD")]
    CloneShallowIfNotConstrainedToWorld = 0,
    #[serde(rename = "CLONE_DATAS_WITH_MOTORS")]
    CloneDatasWithMotors = 1,
    #[serde(rename = "CLONE_FORCE_SHALLOW")]
    CloneForceShallow = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OnDestructionRemapInfo {
    #[serde(rename = "ON_DESTRUCTION_REMAP")]
    OnDestructionRemap = 0,
    #[serde(rename = "ON_DESTRUCTION_REMOVE")]
    OnDestructionRemove = 1,
    #[serde(rename = "ON_DESTRUCTION_RESET_REMOVE")]
    OnDestructionResetRemove = 2,
}
