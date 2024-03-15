//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkLinkAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkLinkAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x255d8164`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkLinkAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum Link`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<Link>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkLinkAttribute, "@name",
    ("type" => Type(Primitive<Link>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Link {
    #[serde(rename = "NONE")]
    None = 0,
    #[serde(rename = "DIRECT_LINK")]
    DirectLink = 1,
    #[serde(rename = "CHILD")]
    Child = 2,
    #[serde(rename = "MESH")]
    Mesh = 3,
    #[serde(rename = "PARENT_NAME")]
    ParentName = 4,
}
