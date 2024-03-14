//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConstraintInstance`
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
    /// # C++ Class Fields Info
    /// -   name:`"owner"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "owner", skip_serializing)]
    Owner(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintModifiers"`
    /// -   type: `struct hkpModifierConstraintAtom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintModifiers")]
    ConstraintModifiers(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"entities"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entities")]
    Entities(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"priority"`
    /// -   type: `enum ConstraintPriority`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "priority")]
    Priority(ConstraintPriority),
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
    DestructionRemapInfo(OnDestructionRemapInfo),
    /// # C++ Class Fields Info
    /// -   name:`"listeners"`
    /// -   type: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType),
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
    Internal(Cow<'a, str>),
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
    ("owner" => Owner(Cow<'de, str>)),
    ("data" => Data(Cow<'de, str>)),
    ("constraintModifiers" => ConstraintModifiers(Cow<'de, str>)),
    ("entities" => Entities(Cow<'de, str>)),
    ("priority" => Priority(ConstraintPriority)),
    ("wantRuntime" => WantRuntime(Primitive<bool>)),
    ("destructionRemapInfo" => DestructionRemapInfo(OnDestructionRemapInfo)),
    ("listeners" => Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("internal" => Internal(Cow<'de, str>)),
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
