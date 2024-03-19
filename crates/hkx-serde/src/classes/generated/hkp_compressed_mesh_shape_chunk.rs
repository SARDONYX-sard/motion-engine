//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShapeChunk`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpCompressedMeshShapeChunk`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x5d0d67bd`
/// -   version: 4
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShapeChunk {
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
    /// -   name:`"indices"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices")]
    Indices(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"stripLengths"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stripLengths")]
    StripLengths(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"materialInfo"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialInfo")]
    MaterialInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reference")]
    Reference(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeChunk, "@name",
    ("offset" => Offset(Primitive<Vector4<f32>>)),
    ("vertices" => Vertices(HkArrayNum<u16>)),
    ("indices" => Indices(HkArrayNum<u16>)),
    ("stripLengths" => StripLengths(HkArrayNum<u16>)),
    ("weldingInfo" => WeldingInfo(HkArrayNum<u16>)),
    ("materialInfo" => MaterialInfo(Primitive<u32>)),
    ("reference" => Reference(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
