//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCylinderShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCylinderShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x3e463c3a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCylinderShape {
    /// # C++ Parent class(`hkpConvexShape`, parent: `hkpSphereRepShape`) field Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius", default)]
    Radius(Primitive<f32>),

    // `hkpSphereRepShape`(Parent class) has no fields

    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", default, skip_serializing)]
    Type(Primitive<Unknown>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"cylRadius"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cylRadius", default)]
    CylRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cylBaseRadiusFactorForHeightFieldCollisions"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cylBaseRadiusFactorForHeightFieldCollisions", default)]
    CylBaseRadiusFactorForHeightFieldCollisions(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexA", default)]
    VertexA(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexB", default)]
    VertexB(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"perpendicular1"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "perpendicular1", default)]
    Perpendicular1(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"perpendicular2"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "perpendicular2", default)]
    Perpendicular2(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCylinderShape, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<Unknown>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("cylRadius" => CylRadius(Primitive<f32>)),
    ("cylBaseRadiusFactorForHeightFieldCollisions" => CylBaseRadiusFactorForHeightFieldCollisions(Primitive<f32>)),
    ("vertexA" => VertexA(Vector4<f32>)),
    ("vertexB" => VertexB(Vector4<f32>)),
    ("perpendicular1" => Perpendicular1(Vector4<f32>)),
    ("perpendicular2" => Perpendicular2(Vector4<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VertexIdEncoding {
    #[serde(rename = "VERTEX_ID_ENCODING_IS_BASE_A_SHIFT")]
    VertexIdEncodingIsBaseAShift = 7,
    #[serde(rename = "VERTEX_ID_ENCODING_SIN_SIGN_SHIFT")]
    VertexIdEncodingSinSignShift = 6,
    #[serde(rename = "VERTEX_ID_ENCODING_COS_SIGN_SHIFT")]
    VertexIdEncodingCosSignShift = 5,
    #[serde(rename = "VERTEX_ID_ENCODING_IS_SIN_LESSER_SHIFT")]
    VertexIdEncodingIsSinLesserShift = 4,
    #[serde(rename = "VERTEX_ID_ENCODING_VALUE_MASK")]
    VertexIdEncodingValueMask = 15,
}
