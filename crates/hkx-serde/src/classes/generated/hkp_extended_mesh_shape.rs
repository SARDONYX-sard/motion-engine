//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"embeddedTrianglesSubpart"`
    /// -   type: `struct hkpExtendedMeshShapeTrianglesSubpart`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "embeddedTrianglesSubpart")]
    EmbeddedTrianglesSubpart(HkpExtendedMeshShapeTrianglesSubpart),
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
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
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
    /// -   type: `hkArray&lt;struct hkpExtendedMeshShapeTrianglesSubpart&gt;`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "trianglesSubparts")]
    TrianglesSubparts(HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart>),
    /// # C++ Class Fields Info
    /// -   name:`"shapesSubparts"`
    /// -   type: `hkArray&lt;struct hkpExtendedMeshShapeShapesSubpart&gt;`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapesSubparts")]
    ShapesSubparts(HkArrayClass<HkpExtendedMeshShapeShapesSubpart>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
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
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padding", skip_serializing)]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShape<'de>, "@name",
    ("embeddedTrianglesSubpart" => EmbeddedTrianglesSubpart(HkpExtendedMeshShapeTrianglesSubpart)),
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("materialClass" => MaterialClass(Primitive<Cow<'de, str>>)),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(Primitive<i32>)),
    ("trianglesSubparts" => TrianglesSubparts(HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart>)),
    ("shapesSubparts" => ShapesSubparts(HkArrayClass<HkpExtendedMeshShapeShapesSubpart>)),
    ("weldingInfo" => WeldingInfo(HkArrayRef<Primitive<u16>>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("cachedNumChildShapes" => CachedNumChildShapes(Primitive<i32>)),
    ("triangleRadius" => TriangleRadius(Primitive<f32>)),
    ("padding" => Padding(Primitive<i32>)),
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubpartType {
    #[serde(rename = "SUBPART_TRIANGLES")]
    SubpartTriangles = 0,
    #[serde(rename = "SUBPART_SHAPE")]
    SubpartShape = 1,
}
