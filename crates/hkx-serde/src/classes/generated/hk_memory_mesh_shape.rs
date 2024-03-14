//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMemoryMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMemoryMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkMeshShape`/`0x9117d62e`
/// - signature: `0xb743a578`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sections"`
    /// -   type: `hkArray&lt;struct hkMeshSectionCinfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sections")]
    Sections(HkArrayClass<HkMeshSectionCinfo>),
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshShape<'de>, "@name",
    ("sections" => Sections(HkArrayClass<HkMeshSectionCinfo>)),
    ("indices16" => Indices16(HkArrayRef<Primitive<u16>>)),
    ("indices32" => Indices32(HkArrayRef<Primitive<u32>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
