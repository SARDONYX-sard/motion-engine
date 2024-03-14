//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCompressedMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 224
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0xa62d5e6e`
/// -   version: 9
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"bitsPerIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitsPerIndex")]
    BitsPerIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"bitsPerWIndex"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitsPerWIndex")]
    BitsPerWIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"wIndexMask"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wIndexMask")]
    WIndexMask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexMask"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexMask")]
    IndexMask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
    /// # C++ Class Fields Info
    /// -   name:`"materialType"`
    /// -   type: `enum MaterialType`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialType")]
    MaterialType(Primitive<MaterialType>),
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"materials16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials16")]
    Materials16(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"materials8"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials8")]
    Materials8(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkArrayVector<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"bigVertices"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bigVertices")]
    BigVertices(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"bigTriangles"`
    /// -   type: `hkArray&lt;struct hkpCompressedMeshShapeBigTriangle&gt;`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bigTriangles")]
    BigTriangles(HkArrayClass<HkpCompressedMeshShapeBigTriangle>),
    /// # C++ Class Fields Info
    /// -   name:`"chunks"`
    /// -   type: `hkArray&lt;struct hkpCompressedMeshShapeChunk&gt;`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chunks")]
    Chunks(HkArrayClass<HkpCompressedMeshShapeChunk>),
    /// # C++ Class Fields Info
    /// -   name:`"convexPieces"`
    /// -   type: `hkArray&lt;struct hkpCompressedMeshShapeConvexPiece&gt;`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieces")]
    ConvexPieces(HkArrayClass<HkpCompressedMeshShapeConvexPiece>),
    /// # C++ Class Fields Info
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "error")]
    Error(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bounds"`
    /// -   type: `struct hkAabb`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bounds")]
    Bounds(HkAabb),
    /// # C++ Class Fields Info
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultCollisionFilterInfo")]
    DefaultCollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"meshMaterials"`
    /// -   type: `void*`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "meshMaterials", skip_serializing)]
    MeshMaterials(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkUint16`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialStriding")]
    MaterialStriding(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 202
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray&lt;struct hkpNamedMeshMaterial&gt;`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedMaterials")]
    NamedMaterials(HkArrayClass<HkpNamedMeshMaterial>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShape<'de>, "@name",
    ("bitsPerIndex" => BitsPerIndex(Primitive<i32>)),
    ("bitsPerWIndex" => BitsPerWIndex(Primitive<i32>)),
    ("wIndexMask" => WIndexMask(Primitive<i32>)),
    ("indexMask" => IndexMask(Primitive<i32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("materialType" => MaterialType(Primitive<MaterialType>)),
    ("materials" => Materials(HkArrayRef<Primitive<u32>>)),
    ("materials16" => Materials16(HkArrayRef<Primitive<u16>>)),
    ("materials8" => Materials8(HkArrayRef<Primitive<u8>>)),
    ("transforms" => Transforms(HkArrayVector<QsTransform<f32>>)),
    ("bigVertices" => BigVertices(HkArrayVector<Vector4<f32>>)),
    ("bigTriangles" => BigTriangles(HkArrayClass<HkpCompressedMeshShapeBigTriangle>)),
    ("chunks" => Chunks(HkArrayClass<HkpCompressedMeshShapeChunk>)),
    ("convexPieces" => ConvexPieces(HkArrayClass<HkpCompressedMeshShapeConvexPiece>)),
    ("error" => Error(Primitive<f32>)),
    ("bounds" => Bounds(HkAabb)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("meshMaterials" => MeshMaterials(Cow<'de, str>)),
    ("materialStriding" => MaterialStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("namedMaterials" => NamedMaterials(HkArrayClass<HkpNamedMeshMaterial>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialType {
    #[serde(rename = "MATERIAL_NONE")]
    MaterialNone = 0,
    #[serde(rename = "MATERIAL_SINGLE_VALUE_PER_CHUNK")]
    MaterialSingleValuePerChunk = 1,
    #[serde(rename = "MATERIAL_ONE_BYTE_PER_TRIANGLE")]
    MaterialOneBytePerTriangle = 2,
    #[serde(rename = "MATERIAL_TWO_BYTES_PER_TRIANGLE")]
    MaterialTwoBytesPerTriangle = 3,
    #[serde(rename = "MATERIAL_FOUR_BYTES_PER_TRIANGLE")]
    MaterialFourBytesPerTriangle = 4,
}
