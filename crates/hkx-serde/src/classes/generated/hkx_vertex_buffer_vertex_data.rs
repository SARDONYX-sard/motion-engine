//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexBufferVertexData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxVertexBufferVertexData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 84
/// -    vtable: false
/// - signature: `0xd72b6fd0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexBufferVertexData {
    /// # C++ Class Fields Info
    /// -   name:`"vectorData"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorData")]
    VectorData(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"floatData"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatData")]
    FloatData(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"uint32Data"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint32Data")]
    Uint32Data(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"uint16Data"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint16Data")]
    Uint16Data(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"uint8Data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint8Data")]
    Uint8Data(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"numVerts"`
    /// -   type: `hkUint32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVerts")]
    NumVerts(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"vectorStride"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorStride")]
    VectorStride(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatStride"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatStride")]
    FloatStride(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"uint32Stride"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint32Stride")]
    Uint32Stride(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"uint16Stride"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint16Stride")]
    Uint16Stride(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"uint8Stride"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "uint8Stride")]
    Uint8Stride(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexBufferVertexData, "@name",
    ("vectorData" => VectorData(HkArrayVector<Vector4<f32>>)),
    ("floatData" => FloatData(HkArrayNum<f32>)),
    ("uint32Data" => Uint32Data(HkArrayNum<u32>)),
    ("uint16Data" => Uint16Data(HkArrayNum<u16>)),
    ("uint8Data" => Uint8Data(HkArrayNum<u8>)),
    ("numVerts" => NumVerts(Primitive<u32>)),
    ("vectorStride" => VectorStride(Primitive<u32>)),
    ("floatStride" => FloatStride(Primitive<u32>)),
    ("uint32Stride" => Uint32Stride(Primitive<u32>)),
    ("uint16Stride" => Uint16Stride(Primitive<u32>)),
    ("uint8Stride" => Uint8Stride(Primitive<u32>)),
}
