//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSingleShapeContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSingleShapeContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: true
/// -    parent: `hkpShapeContainer`/`0xe0708a00`
/// - signature: `0x73aa1d38`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSingleShapeContainer<'a> {
    // `hkpShapeContainer`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape", default)]
    ChildShape(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSingleShapeContainer<'de>, "@name",
    ("childShape" => ChildShape(Primitive<Cow<'de, str>>)),
}
