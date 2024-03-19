//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMultiRayShapeRay`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpMultiRayShapeRay`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xffdc0b65`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMultiRayShapeRay {
    /// # C++ Class Fields Info
    /// -   name:`"start"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "start")]
    Start(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"end"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "end")]
    End(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMultiRayShapeRay, "@name",
    ("start" => Start(Primitive<Vector4<f32>>)),
    ("end" => End(Primitive<Vector4<f32>>)),
}
