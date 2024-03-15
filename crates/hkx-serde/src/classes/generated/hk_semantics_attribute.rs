//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkSemanticsAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkSemanticsAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x837099c3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSemanticsAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum Semantics`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<Semantics>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkSemanticsAttribute, "@name",
    ("type" => Type(Primitive<Semantics>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Semantics {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "DISTANCE")]
    Distance = 1,
    #[serde(rename = "ANGLE")]
    Angle = 2,
    #[serde(rename = "NORMAL")]
    Normal = 3,
    #[serde(rename = "POSITION")]
    Position = 4,
    #[serde(rename = "COSINE_ANGLE")]
    CosineAngle = 5,
}
