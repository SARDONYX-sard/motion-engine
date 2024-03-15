//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshSectionCinfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMeshSectionCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x6075f3ff`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshSectionCinfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `struct hkMeshMaterial*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"primitiveType"`
    /// -   type: `enum PrimitiveType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "primitiveType")]
    PrimitiveType(Primitive<PrimitiveType>),
    /// # C++ Class Fields Info
    /// -   name:`"numPrimitives"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numPrimitives")]
    NumPrimitives(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexType"`
    /// -   type: `enum MeshSectionIndexType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType")]
    IndexType(Primitive<MeshSectionIndexType>),
    /// # C++ Class Fields Info
    /// -   name:`"indices"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indices", skip_serializing)]
    Indices(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexStartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStartIndex")]
    VertexStartIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshSectionCinfo<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Primitive<Cow<'de, str>>)),
    ("material" => Material(Primitive<Cow<'de, str>>)),
    ("primitiveType" => PrimitiveType(Primitive<PrimitiveType>)),
    ("numPrimitives" => NumPrimitives(Primitive<i32>)),
    ("indexType" => IndexType(Primitive<MeshSectionIndexType>)),
    ("indices" => Indices(Primitive<Cow<'de, str>>)),
    ("vertexStartIndex" => VertexStartIndex(Primitive<i32>)),
    ("transformIndex" => TransformIndex(Primitive<i32>)),
}
