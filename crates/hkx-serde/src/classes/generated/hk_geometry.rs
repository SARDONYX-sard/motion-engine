//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkGeometry`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkGeometry`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0x98dd8bdc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkGeometry {
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"triangles"`
    /// -   type: `hkArray<struct hkGeometryTriangle>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangles")]
    Triangles(HkArrayClass<HkGeometryTriangle>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkGeometry, "@name",
    ("vertices" => Vertices(HkArrayVector<Vector4<f32>>)),
    ("triangles" => Triangles(HkArrayClass<HkGeometryTriangle>)),
}
