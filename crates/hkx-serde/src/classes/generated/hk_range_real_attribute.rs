//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRangeRealAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkRangeRealAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x949db24f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkRangeRealAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"absmin"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absmin")]
    Absmin(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"absmax"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absmax")]
    Absmax(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"softmin"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "softmin")]
    Softmin(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"softmax"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "softmax")]
    Softmax(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRangeRealAttribute, "@name",
    ("absmin" => Absmin(Primitive<f32>)),
    ("absmax" => Absmax(Primitive<f32>)),
    ("softmin" => Softmin(Primitive<f32>)),
    ("softmax" => Softmax(Primitive<f32>)),
}
