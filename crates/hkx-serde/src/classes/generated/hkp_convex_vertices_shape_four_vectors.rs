//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexVerticesShapeFourVectors`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpConvexVerticesShapeFourVectors`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x3d80c5bf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexVerticesShapeFourVectors {
    /// # C++ Class Fields Info
    /// -   name:`"x"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "x")]
    X(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"y"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "y")]
    Y(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"z"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "z")]
    Z(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesShapeFourVectors, "@name",
    ("x" => X(Primitive<Vector4<f32>>)),
    ("y" => Y(Primitive<Vector4<f32>>)),
    ("z" => Z(Primitive<Vector4<f32>>)),
}
