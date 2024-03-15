//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexVerticesShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexVerticesShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x28726ad8`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexVerticesShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotatedVertices"`
    /// -   type: `hkArray&lt;struct hkpConvexVerticesShapeFourVectors&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotatedVertices")]
    RotatedVertices(HkArrayClass<HkpConvexVerticesShapeFourVectors>),
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"externalObject"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "externalObject", skip_serializing)]
    ExternalObject(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"getFaceNormals"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "getFaceNormals", skip_serializing)]
    GetFaceNormals(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"planeEquations"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planeEquations")]
    PlaneEquations(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"connectivity"`
    /// -   type: `struct hkpConvexVerticesConnectivity*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "connectivity")]
    Connectivity(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesShape<'de>, "@name",
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("rotatedVertices" => RotatedVertices(HkArrayClass<HkpConvexVerticesShapeFourVectors>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("externalObject" => ExternalObject(Primitive<Cow<'de, str>>)),
    ("getFaceNormals" => GetFaceNormals(Primitive<Cow<'de, str>>)),
    ("planeEquations" => PlaneEquations(HkArrayVector<Vector4<f32>>)),
    ("connectivity" => Connectivity(Primitive<Cow<'de, str>>)),
}
