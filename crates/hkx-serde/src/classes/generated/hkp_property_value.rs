//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPropertyValue`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpPropertyValue`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xc75925aa`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPropertyValue {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkUint64`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<u64>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPropertyValue, "@name",
    ("data" => Data(Primitive<u64>)),
}
