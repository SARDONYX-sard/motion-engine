//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPostFinishAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkPostFinishAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x903abb2c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPostFinishAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"postFinishFunction"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "postFinishFunction", skip_serializing)]
    PostFinishFunction(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPostFinishAttribute<'de>, "@name",
    ("postFinishFunction" => PostFinishFunction(Cow<'de, str>)),
}
