//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedSampledHeightFieldShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCompressedSampledHeightFieldShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkpSampledHeightFieldShape`/`0x11213421`
/// - signature: `0x97b6e143`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedSampledHeightFieldShape {
    /// # C++ Class Fields Info
    /// -   name:`"storage"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "storage")]
    Storage(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"triangleFlip"`
    /// -   type: `hkBool`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleFlip")]
    TriangleFlip(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"scale"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scale")]
    Scale(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedSampledHeightFieldShape, "@name",
    ("storage" => Storage(HkArrayRef<Primitive<u16>>)),
    ("triangleFlip" => TriangleFlip(Primitive<bool>)),
    ("offset" => Offset(Primitive<f32>)),
    ("scale" => Scale(Primitive<f32>)),
}
