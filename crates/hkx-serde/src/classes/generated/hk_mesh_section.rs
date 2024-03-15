//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshSection`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMeshSection`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: false
/// - signature: `0x1893c365`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshSection<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"primitiveType"`
    /// -   type: `enum PrimitiveType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "primitiveType", default)]
    PrimitiveType(Primitive<PrimitiveType>),
    /// # C++ Class Fields Info
    /// -   name:`"numPrimitives"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numPrimitives", default)]
    NumPrimitives(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numIndices"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numIndices", default)]
    NumIndices(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexStartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStartIndex", default)]
    VertexStartIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex", default)]
    TransformIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexType"`
    /// -   type: `enum MeshSectionIndexType`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType", default)]
    IndexType(Primitive<MeshSectionIndexType>),
    /// # C++ Class Fields Info
    /// -   name:`"indices"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indices", default, skip_serializing)]
    Indices(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer", default)]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `struct hkMeshMaterial*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material", default)]
    Material(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"sectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectionIndex", default)]
    SectionIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshSection<'de>, "@name",
    ("primitiveType" => PrimitiveType(Primitive<PrimitiveType>)),
    ("numPrimitives" => NumPrimitives(Primitive<i32>)),
    ("numIndices" => NumIndices(Primitive<i32>)),
    ("vertexStartIndex" => VertexStartIndex(Primitive<i32>)),
    ("transformIndex" => TransformIndex(Primitive<i32>)),
    ("indexType" => IndexType(Primitive<MeshSectionIndexType>)),
    ("indices" => Indices(Primitive<Cow<'de, str>>)),
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("material" => Material(Primitive<Cow<'de, str>>)),
    ("sectionIndex" => SectionIndex(Primitive<i32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeshSectionIndexType {
    #[serde(rename = "INDEX_TYPE_NONE")]
    IndexTypeNone = 0,
    #[serde(rename = "INDEX_TYPE_UINT16")]
    IndexTypeUint16 = 1,
    #[serde(rename = "INDEX_TYPE_UINT32")]
    IndexTypeUint32 = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrimitiveType {
    #[serde(rename = "PRIMITIVE_TYPE_UNKNOWN")]
    PrimitiveTypeUnknown = 0,
    #[serde(rename = "PRIMITIVE_TYPE_POINT_LIST")]
    PrimitiveTypePointList = 1,
    #[serde(rename = "PRIMITIVE_TYPE_LINE_LIST")]
    PrimitiveTypeLineList = 2,
    #[serde(rename = "PRIMITIVE_TYPE_TRIANGLE_LIST")]
    PrimitiveTypeTriangleList = 3,
    #[serde(rename = "PRIMITIVE_TYPE_TRIANGLE_STRIP")]
    PrimitiveTypeTriangleStrip = 4,
}
