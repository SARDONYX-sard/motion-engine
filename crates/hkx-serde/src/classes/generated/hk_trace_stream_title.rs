//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTraceStreamTitle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkTraceStreamTitle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x6a4ca82c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTraceStreamTitle {
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `hkChar[32]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(CStyleArray<[char; 32]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTraceStreamTitle, "@name",
    ("value" => Value(CStyleArray<[char; 32]>)),
}
