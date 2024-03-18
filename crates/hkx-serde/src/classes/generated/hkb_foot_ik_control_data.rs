//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbFootIkControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xa111b704`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkControlData {
    /// # C++ Class Fields Info
    /// -   name:`"gains"`
    /// -   type: `struct hkbFootIkGains`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "gains")]
    Gains(SingleClass<HkbFootIkGains>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkControlData, "@name",
    ("gains" => Gains(SingleClass<HkbFootIkGains>)),
}
