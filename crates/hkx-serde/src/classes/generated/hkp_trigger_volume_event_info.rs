//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriggerVolumeEventInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTriggerVolumeEventInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xeb60f431`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriggerVolumeEventInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sortValue"`
    /// -   type: `hkUint64`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sortValue")]
    SortValue(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"body"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "body")]
    Body(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"operation"`
    /// -   type: `enum Operation`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "operation")]
    Operation(Primitive<Operation>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriggerVolumeEventInfo<'de>, "@name",
    ("sortValue" => SortValue(Primitive<u64>)),
    ("body" => Body(Primitive<Cow<'de, str>>)),
    ("operation" => Operation(Primitive<Operation>)),
}
