//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryResourceContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMemoryResourceContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkResourceContainer`/`0x4e94146`
/// - signature: `0x4762f92a`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryResourceContainer<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"parent"`
    /// -   type: `struct hkMemoryResourceContainer*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parent", skip_serializing)]
    Parent(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"resourceHandles"`
    /// -   type: `hkArray&lt;hkMemoryResourceHandle*&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resourceHandles")]
    ResourceHandles(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkMemoryResourceContainer*&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceContainer<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
    ("resourceHandles" => ResourceHandles(HkArrayRef<Cow<'de, str>>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
}
