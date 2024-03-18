//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkHalf8`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkHalf8`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x7684dc80`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkHalf8 {
    /// # C++ Class Fields Info
    /// -   name:`"quad"`
    /// -   type: `hkHalf[8]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "quad")]
    Quad(CStyleArray<[f32; 8]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkHalf8, "@name",
    ("quad" => Quad(CStyleArray<[f32; 8]>)),
}
