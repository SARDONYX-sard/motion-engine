//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpDisplayBindingDataPhysicsSystem`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpDisplayBindingDataPhysicsSystem`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc8ae86a7`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisplayBindingDataPhysicsSystem<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"bindings"`
    /// -   type: `hkArray&lt;hkpDisplayBindingDataRigidBody*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindings")]
    Bindings(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"system"`
    /// -   type: `struct hkpPhysicsSystem*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "system")]
    System(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingDataPhysicsSystem<'de>, "@name",
    ("bindings" => Bindings(HkArrayRef<Cow<'de, str>>)),
    ("system" => System(Cow<'de, str>)),
}
