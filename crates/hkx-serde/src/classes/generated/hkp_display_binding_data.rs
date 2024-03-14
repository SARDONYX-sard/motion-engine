//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpDisplayBindingData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpDisplayBindingData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xdc46c906`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisplayBindingData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"rigidBodyBindings"`
    /// -   type: `hkArray&lt;hkpDisplayBindingDataRigidBody*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBodyBindings")]
    RigidBodyBindings(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"physicsSystemBindings"`
    /// -   type: `hkArray&lt;hkpDisplayBindingDataPhysicsSystem*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "physicsSystemBindings")]
    PhysicsSystemBindings(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingData<'de>, "@name",
    ("rigidBodyBindings" => RigidBodyBindings(HkArrayRef<Cow<'de, str>>)),
    ("physicsSystemBindings" => PhysicsSystemBindings(HkArrayRef<Cow<'de, str>>)),
}
