//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbReferencePoseGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbReferencePoseGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x26a5675a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbReferencePoseGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "skeleton", skip_serializing)]
    Skeleton(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbReferencePoseGenerator<'de>, "@name",
    ("skeleton" => Skeleton(Cow<'de, str>)),
}
