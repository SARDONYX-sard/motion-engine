//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpPlaneShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPlaneShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpHeightFieldShape`/`0xe7eca7eb`
/// - signature: `0xc36bbd30`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPlaneShape {
    /// # C++ Class Fields Info
    /// -   name:`"plane"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "plane")]
    Plane(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPlaneShape, "@name",
    ("plane" => Plane(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
}
