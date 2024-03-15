//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSRagdollContactListenerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSRagdollContactListenerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x8003d8ce`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsRagdollContactListenerModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"contactEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactEvent")]
    ContactEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"throwEvent"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "throwEvent", skip_serializing)]
    ThrowEvent(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"ragdollRigidBodies"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ragdollRigidBodies", skip_serializing)]
    RagdollRigidBodies(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsRagdollContactListenerModifier<'de>, "@name",
    ("contactEvent" => ContactEvent(HkbEventProperty)),
    ("bones" => Bones(Primitive<Cow<'de, str>>)),
    ("throwEvent" => ThrowEvent(Primitive<bool>)),
    ("ragdollRigidBodies" => RagdollRigidBodies(HkArrayRef<Cow<'de, str>>)),
}
