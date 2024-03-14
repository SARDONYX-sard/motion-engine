//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxSparselyAnimatedEnum`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxSparselyAnimatedEnum`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkxSparselyAnimatedInt`/`0xca961951`
/// - signature: `0x68a47b64`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSparselyAnimatedEnum<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"enum"`
    /// -   type: `struct hkxEnum*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enum")]
    Enum(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedEnum<'de>, "@name",
    ("enum" => Enum(Cow<'de, str>)),
}
