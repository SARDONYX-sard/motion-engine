//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintChainInstance`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintChainInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkpConstraintInstance`/`0x34eba5f`
/// - signature: `0x7a490753`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintChainInstance<'a> {
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"owner"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "owner", skip_serializing)]
    Owner(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"data"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"constraintModifiers"`
    /// -   type: `struct hkpModifierConstraintAtom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintModifiers")]
    ConstraintModifiers(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"entities"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entities")]
    Entities(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"priority"`
    /// -   type: `enum ConstraintPriority`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "priority")]
    Priority(Primitive<ConstraintPriority>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"wantRuntime"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantRuntime")]
    WantRuntime(Primitive<bool>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"destructionRemapInfo"`
    /// -   type: `enum OnDestructionRemapInfo`
    /// - offset: 30
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "destructionRemapInfo")]
    DestructionRemapInfo(Primitive<OnDestructionRemapInfo>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"listeners"`
    /// -   type: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkpConstraintInstanceSmallArraySerializeOverrideType<'a>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"internal"`
    /// -   type: `void*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internal", skip_serializing)]
    Internal(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpConstraintInstance` => parent: `hkReferencedObject`) field Info
    /// -   name:`"uid"`
    /// -   type: `hkUint32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "uid", skip_serializing)]
    Uid(Primitive<u32>),

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
    /// -   name:`"chainedEntities"`
    /// -   type: `hkArray&lt;hkpEntity*&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chainedEntities")]
    ChainedEntities(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"action"`
    /// -   type: `struct hkpConstraintChainInstanceAction*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "action")]
    Action(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintChainInstance<'de>, "@name",
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
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("chainedEntities" => ChainedEntities(HkArrayRef<Cow<'de, str>>)),
    ("action" => Action(Primitive<Cow<'de, str>>)),
}
