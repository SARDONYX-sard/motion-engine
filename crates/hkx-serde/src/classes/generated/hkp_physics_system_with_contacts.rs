//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpPhysicsSystemWithContacts`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPhysicsSystemWithContacts`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkpPhysicsSystem`/`0xff724c17`
/// - signature: `0xd0fd4bbe`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPhysicsSystemWithContacts<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"contacts"`
    /// -   type: `hkArray&lt;hkpSerializedAgentNnEntry*&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contacts")]
    Contacts(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhysicsSystemWithContacts<'de>, "@name",
    ("contacts" => Contacts(HkArrayRef<Cow<'de, str>>)),
}
