//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbEventPayloadList`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventPayloadList`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkbEventPayload`/`0xda8c7d7d`
/// - signature: `0x3d2dbd34`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventPayloadList<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"payloads"`
    /// -   type: `hkArray&lt;hkbEventPayload*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "payloads")]
    Payloads(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventPayloadList<'de>, "@name",
    ("payloads" => Payloads(HkArrayRef<Cow<'de, str>>)),
}
