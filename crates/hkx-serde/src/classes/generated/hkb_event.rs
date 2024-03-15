//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEvent`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEvent`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkbEventBase`/`0x76bddb31`
/// - signature: `0x3e0fd810`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvent<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sender"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "sender", skip_serializing)]
    Sender(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvent<'de>, "@name",
    ("sender" => Sender(Primitive<Cow<'de, str>>)),
}
