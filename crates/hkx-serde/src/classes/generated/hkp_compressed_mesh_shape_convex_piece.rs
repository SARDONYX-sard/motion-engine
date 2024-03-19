//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShapeConvexPiece`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpCompressedMeshShapeConvexPiece`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x385bb842`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShapeConvexPiece {
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"faceVertices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "faceVertices")]
    FaceVertices(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"faceOffsets"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "faceOffsets")]
    FaceOffsets(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reference")]
    Reference(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeConvexPiece, "@name",
    ("offset" => Offset(Primitive<Vector4<f32>>)),
    ("vertices" => Vertices(HkArrayNum<u16>)),
    ("faceVertices" => FaceVertices(HkArrayNum<u8>)),
    ("faceOffsets" => FaceOffsets(HkArrayNum<u16>)),
    ("reference" => Reference(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
