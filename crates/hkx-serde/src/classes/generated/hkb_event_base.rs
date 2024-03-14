//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbEventBase`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventBase`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x76bddb31`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventBase<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "id")]
    Id(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"payload"`
    /// -   type: `struct hkbEventPayload*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "payload")]
    Payload(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventBase<'de>, "@name",
    ("id" => Id(Primitive<i32>)),
    ("payload" => Payload(Cow<'de, str>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemEventIds {
    #[serde(rename = "EVENT_ID_NULL")]
    EventIdNull = -1,
}
