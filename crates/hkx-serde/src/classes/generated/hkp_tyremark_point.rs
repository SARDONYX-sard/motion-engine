//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTyremarkPoint`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTyremarkPoint`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x6bb7c5e8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTyremarkPoint {
    /// # C++ Class Fields Info
    /// -   name:`"pointLeft"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointLeft")]
    PointLeft(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"pointRight"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointRight")]
    PointRight(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarkPoint, "@name",
    ("pointLeft" => PointLeft(Vector4<f32>)),
    ("pointRight" => PointRight(Vector4<f32>)),
}
