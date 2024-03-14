//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpMultiRayShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMultiRayShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0xea2e7ec9`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMultiRayShape {
    /// # C++ Class Fields Info
    /// -   name:`"rays"`
    /// -   type: `hkArray&lt;struct hkpMultiRayShapeRay&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rays")]
    Rays(HkArrayClass<HkpMultiRayShapeRay>),
    /// # C++ Class Fields Info
    /// -   name:`"rayPenetrationDistance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rayPenetrationDistance")]
    RayPenetrationDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMultiRayShape, "@name",
    ("rays" => Rays(HkArrayClass<HkpMultiRayShapeRay>)),
    ("rayPenetrationDistance" => RayPenetrationDistance(Primitive<f32>)),
}
