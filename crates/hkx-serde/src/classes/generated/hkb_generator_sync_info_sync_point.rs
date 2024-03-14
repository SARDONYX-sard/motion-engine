//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbGeneratorSyncInfoSyncPoint`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbGeneratorSyncInfoSyncPoint`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xb597cf92`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorSyncInfoSyncPoint {
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "id")]
    Id(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorSyncInfoSyncPoint, "@name",
    ("id" => Id(Primitive<i32>)),
    ("time" => Time(Primitive<f32>)),
}
