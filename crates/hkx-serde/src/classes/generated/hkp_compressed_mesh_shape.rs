//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShape<'a> {
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disableWelding")]
    DisableWelding(Primitive<bool>),
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collectionType")]
    CollectionType(Primitive<CollectionType>),

    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
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
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"materials16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials16")]
    Materials16(HkArrayNum<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"materials8"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials8")]
    Materials8(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkArrayMatrix3<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"bigVertices"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bigVertices")]
    BigVertices(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"bigTriangles"`
    /// -   type: `hkArray<struct hkpCompressedMeshShapeBigTriangle>`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bigTriangles")]
    BigTriangles(HkArrayClass<HkpCompressedMeshShapeBigTriangle>),
    /// # C++ Class Fields Info
    /// -   name:`"chunks"`
    /// -   type: `hkArray<struct hkpCompressedMeshShapeChunk>`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chunks")]
    Chunks(HkArrayClass<HkpCompressedMeshShapeChunk>),
    /// # C++ Class Fields Info
    /// -   name:`"convexPieces"`
    /// -   type: `hkArray<struct hkpCompressedMeshShapeConvexPiece>`
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
    Bounds(SingleClass<HkAabb>),
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
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "meshMaterials", skip_serializing)]
    MeshMaterials(Primitive<Cow<'a, str>>),
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
    /// -   type: `hkArray<struct hkpNamedMeshMaterial>`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedMaterials")]
    NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShape<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bitsPerIndex" => BitsPerIndex(Primitive<i32>)),
    ("bitsPerWIndex" => BitsPerWIndex(Primitive<i32>)),
    ("wIndexMask" => WIndexMask(Primitive<i32>)),
    ("indexMask" => IndexMask(Primitive<i32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("materialType" => MaterialType(Primitive<MaterialType>)),
    ("materials" => Materials(HkArrayNum<u32>)),
    ("materials16" => Materials16(HkArrayNum<u16>)),
    ("materials8" => Materials8(HkArrayNum<u8>)),
    ("transforms" => Transforms(HkArrayMatrix3<QsTransform<f32>>)),
    ("bigVertices" => BigVertices(HkArrayVector<Vector4<f32>>)),
    ("bigTriangles" => BigTriangles(HkArrayClass<HkpCompressedMeshShapeBigTriangle>)),
    ("chunks" => Chunks(HkArrayClass<HkpCompressedMeshShapeChunk>)),
    ("convexPieces" => ConvexPieces(HkArrayClass<HkpCompressedMeshShapeConvexPiece>)),
    ("error" => Error(Primitive<f32>)),
    ("bounds" => Bounds(SingleClass<HkAabb>)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("meshMaterials" => MeshMaterials(Primitive<Cow<'de, str>>)),
    ("materialStriding" => MaterialStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("namedMaterials" => NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'de>>)),
}

#[allow(clippy::enum_variant_names)]
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
