//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMonitorStreamStringMap`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMonitorStreamStringMap`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xc4d3a8b4`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamStringMap {
    /// # C++ Class Fields Info
    /// -   name:`"map"`
    /// -   type: `hkArray&lt;struct hkMonitorStreamStringMapStringMap&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "map")]
    Map(HkArrayClass<HkMonitorStreamStringMapStringMap>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamStringMap, "@name",
    ("map" => Map(HkArrayClass<HkMonitorStreamStringMapStringMap>)),
}
