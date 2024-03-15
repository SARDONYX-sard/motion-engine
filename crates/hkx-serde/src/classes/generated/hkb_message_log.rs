//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbMessageLog`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbMessageLog`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x26a196c5`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbMessageLog<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"messages"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "messages", skip_serializing)]
    Messages(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"maxMessages"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "maxMessages", skip_serializing)]
    MaxMessages(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbMessageLog<'de>, "@name",
    ("messages" => Messages(Primitive<Cow<'de, str>>)),
    ("maxMessages" => MaxMessages(Primitive<i32>)),
}
