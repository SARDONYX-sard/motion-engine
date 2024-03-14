//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkArrayTypeAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkArrayTypeAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xd404a39a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkArrayTypeAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum ArrayType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<ArrayType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkArrayTypeAttribute, "@name",
    ("type" => Type(Primitive<ArrayType>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArrayType {
    #[serde(rename = "NONE")]
    None = 0,
    #[serde(rename = "POINTSOUP")]
    Pointsoup = 1,
    #[serde(rename = "ENTITIES")]
    Entities = 2,
}
