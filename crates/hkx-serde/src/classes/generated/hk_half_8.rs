//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkHalf8`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkHalf8`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x7684dc80`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkHalf8 {
    /// # C++ Class Fields Info
    /// -   name:`"quad"`
    /// -   type: `hkHalf[8]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "quad")]
    Quad([Primitive<f32>; 8]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkHalf8, "@name",
    ("quad" => Quad([Primitive<f32>; 8])),
}
