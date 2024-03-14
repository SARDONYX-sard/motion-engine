//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRemoveTerminalsMoppModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpRemoveTerminalsMoppModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x91367f03`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRemoveTerminalsMoppModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"removeInfo"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "removeInfo")]
    RemoveInfo(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"tempShapesToRemove"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "tempShapesToRemove", skip_serializing)]
    TempShapesToRemove(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRemoveTerminalsMoppModifier<'de>, "@name",
    ("removeInfo" => RemoveInfo(HkArrayRef<Primitive<u32>>)),
    ("tempShapesToRemove" => TempShapesToRemove(Cow<'de, str>)),
}
