//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x7375cae3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value", default)]
    Value(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAttribute<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("value" => Value(Primitive<Cow<'de, str>>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Hint {
    #[serde(rename = "HINT_NONE")]
    HintNone = 0,
    #[serde(rename = "HINT_IGNORE")]
    HintIgnore = 1,
    #[serde(rename = "HINT_TRANSFORM")]
    HintTransform = 2,
    #[serde(rename = "HINT_SCALE")]
    HintScale = 4,
    #[serde(rename = "HINT_TRANSFORM_AND_SCALE")]
    HintTransformAndScale = 6,
    #[serde(rename = "HINT_FLIP")]
    HintFlip = 8,
}
