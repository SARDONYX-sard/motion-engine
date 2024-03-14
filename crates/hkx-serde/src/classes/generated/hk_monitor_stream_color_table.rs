//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMonitorStreamColorTable`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMonitorStreamColorTable`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x79e53e85`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamColorTable {
    /// # C++ Class Fields Info
    /// -   name:`"colorPairs"`
    /// -   type: `hkArray&lt;struct hkMonitorStreamColorTableColorPair&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "colorPairs")]
    ColorPairs(HkArrayClass<HkMonitorStreamColorTableColorPair>),
    /// # C++ Class Fields Info
    /// -   name:`"defaultColor"`
    /// -   type: `hkUint32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultColor")]
    DefaultColor(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamColorTable, "@name",
    ("colorPairs" => ColorPairs(HkArrayClass<HkMonitorStreamColorTableColorPair>)),
    ("defaultColor" => DefaultColor(Primitive<u32>)),
}
