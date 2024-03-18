//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpExtendedMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 240
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0x177114a2`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShape<'a> {
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

    /// # C++ Class Fields Info
    /// -   name:`"embeddedTrianglesSubpart"`
    /// -   type: `struct hkpExtendedMeshShapeTrianglesSubpart`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "embeddedTrianglesSubpart")]
    EmbeddedTrianglesSubpart(SingleClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"materialClass"`
    /// -   type: `void*`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialClass", skip_serializing)]
    MaterialClass(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBitsForSubpartIndex")]
    NumBitsForSubpartIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"trianglesSubparts"`
    /// -   type: `hkArray<struct hkpExtendedMeshShapeTrianglesSubpart>`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "trianglesSubparts")]
    TrianglesSubparts(HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"shapesSubparts"`
    /// -   type: `hkArray<struct hkpExtendedMeshShapeShapesSubpart>`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapesSubparts")]
    ShapesSubparts(HkArrayClass<HkpExtendedMeshShapeShapesSubpart<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
    /// # C++ Class Fields Info
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultCollisionFilterInfo")]
    DefaultCollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"cachedNumChildShapes"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cachedNumChildShapes")]
    CachedNumChildShapes(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"triangleRadius"`
    /// -   type: `hkReal`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleRadius")]
    TriangleRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "padding", skip_serializing)]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShape<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("embeddedTrianglesSubpart" => EmbeddedTrianglesSubpart(SingleClass<HkpExtendedMeshShapeTrianglesSubpart<'de>>)),
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("materialClass" => MaterialClass(Primitive<Cow<'de, str>>)),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(Primitive<i32>)),
    ("trianglesSubparts" => TrianglesSubparts(HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart<'de>>)),
    ("shapesSubparts" => ShapesSubparts(HkArrayClass<HkpExtendedMeshShapeShapesSubpart<'de>>)),
    ("weldingInfo" => WeldingInfo(HkArrayRef<Primitive<u16>>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("cachedNumChildShapes" => CachedNumChildShapes(Primitive<i32>)),
    ("triangleRadius" => TriangleRadius(Primitive<f32>)),
    ("padding" => Padding(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexStridingType {
    #[serde(rename = "INDICES_INVALID")]
    IndicesInvalid = 0,
    #[serde(rename = "INDICES_INT8")]
    IndicesInt8 = 1,
    #[serde(rename = "INDICES_INT16")]
    IndicesInt16 = 2,
    #[serde(rename = "INDICES_INT32")]
    IndicesInt32 = 3,
    #[serde(rename = "INDICES_MAX_ID")]
    IndicesMaxId = 4,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialIndexStridingType {
    #[serde(rename = "MATERIAL_INDICES_INVALID")]
    MaterialIndicesInvalid = 0,
    #[serde(rename = "MATERIAL_INDICES_INT8")]
    MaterialIndicesInt8 = 1,
    #[serde(rename = "MATERIAL_INDICES_INT16")]
    MaterialIndicesInt16 = 2,
    #[serde(rename = "MATERIAL_INDICES_MAX_ID")]
    MaterialIndicesMaxId = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubpartType {
    #[serde(rename = "SUBPART_TRIANGLES")]
    SubpartTriangles = 0,
    #[serde(rename = "SUBPART_SHAPE")]
    SubpartShape = 1,
}
