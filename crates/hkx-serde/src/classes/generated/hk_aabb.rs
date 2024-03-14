//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkAabb`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkAabb`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x4a948b16`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAabb {
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "min")]
    Min(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"max"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "max")]
    Max(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAabb, "@name",
    ("min" => Min(Vector4<f32>)),
    ("max" => Max(Vector4<f32>)),
}
