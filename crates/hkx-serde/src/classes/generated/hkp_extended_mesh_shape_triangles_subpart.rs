//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShapeTrianglesSubpart`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpExtendedMeshShapeTrianglesSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// -    parent: `hkpExtendedMeshShapeSubpart`/`0xf4608207`
/// - signature: `0x44c32df6`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeTrianglesSubpart<'a> {
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum SubpartType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<SubpartType>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MaterialIndexStridingType`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MaterialIndexStridingType>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialStriding", skip_serializing)]
    MaterialStriding(Primitive<i16>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<u16>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart`, parent: `None`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),

    /// # C++ Class Fields Info
    /// -   name:`"numTriangleShapes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numTriangleShapes")]
    NumTriangleShapes(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexBase"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "vertexBase", skip_serializing)]
    VertexBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexBase"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indexBase", skip_serializing)]
    IndexBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStriding")]
    VertexStriding(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"triangleOffset"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleOffset")]
    TriangleOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexStriding")]
    IndexStriding(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"stridingType"`
    /// -   type: `enum IndexStridingType`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stridingType")]
    StridingType(Primitive<IndexStridingType>),
    /// # C++ Class Fields Info
    /// -   name:`"flipAlternateTriangles"`
    /// -   type: `hkInt8`
    /// - offset: 47
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flipAlternateTriangles")]
    FlipAlternateTriangles(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"extrusion"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrusion")]
    Extrusion(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkQsTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(QsTransform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeTrianglesSubpart<'de>, "@name",
    ("type" => Type(Primitive<SubpartType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MaterialIndexStridingType>)),
    ("materialStriding" => MaterialStriding(Primitive<i16>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("numTriangleShapes" => NumTriangleShapes(Primitive<i32>)),
    ("vertexBase" => VertexBase(Primitive<Cow<'de, str>>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("indexBase" => IndexBase(Primitive<Cow<'de, str>>)),
    ("vertexStriding" => VertexStriding(Primitive<u16>)),
    ("triangleOffset" => TriangleOffset(Primitive<i32>)),
    ("indexStriding" => IndexStriding(Primitive<u16>)),
    ("stridingType" => StridingType(Primitive<IndexStridingType>)),
    ("flipAlternateTriangles" => FlipAlternateTriangles(Primitive<i8>)),
    ("extrusion" => Extrusion(Vector4<f32>)),
    ("transform" => Transform(QsTransform<f32>)),
}
