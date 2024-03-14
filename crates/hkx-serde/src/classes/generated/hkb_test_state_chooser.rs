//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTestStateChooser`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbTestStateChooser`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkbStateChooser`/`0xda8c7d7d`
/// - signature: `0xc0fcc436`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTestStateChooser<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"int"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "int")]
    Int(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"real"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "real")]
    Real(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"string"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "string")]
    String(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTestStateChooser<'de>, "@name",
    ("int" => Int(Primitive<i32>)),
    ("real" => Real(Primitive<f32>)),
    ("string" => String(Primitive<Cow<'de, str>>)),
}
