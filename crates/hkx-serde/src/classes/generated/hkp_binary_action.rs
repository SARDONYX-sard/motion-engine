//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpBinaryAction`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBinaryAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpAction`/`0xbdf70a51`
/// - signature: `0xc00f3403`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBinaryAction<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"entityA"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entityA")]
    EntityA(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"entityB"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "entityB")]
    EntityB(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBinaryAction<'de>, "@name",
    ("entityA" => EntityA(Cow<'de, str>)),
    ("entityB" => EntityB(Cow<'de, str>)),
}
