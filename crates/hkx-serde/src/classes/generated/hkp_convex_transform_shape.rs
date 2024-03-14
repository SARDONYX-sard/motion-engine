//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConvexTransformShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexTransformShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpConvexTransformShapeBase`/`0xfbd72f9`
/// - signature: `0xae3e5017`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexTransformShape {
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Transform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexTransformShape, "@name",
    ("transform" => Transform(Transform<f32>)),
}
