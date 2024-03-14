//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpTriangleShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTriangleShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x95ad1a25`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriangleShape {
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
    /// # C++ Class Fields Info
    /// -   name:`"isExtruded"`
    /// -   type: `hkUint8`
    /// - offset: 23
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isExtruded")]
    IsExtruded(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexA")]
    VertexA(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexB")]
    VertexB(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexC"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexC")]
    VertexC(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extrusion"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrusion")]
    Extrusion(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriangleShape, "@name",
    ("weldingInfo" => WeldingInfo(Primitive<u16>)),
    ("weldingType" => WeldingType(WeldingType)),
    ("isExtruded" => IsExtruded(Primitive<u8>)),
    ("vertexA" => VertexA(Vector4<f32>)),
    ("vertexB" => VertexB(Vector4<f32>)),
    ("vertexC" => VertexC(Vector4<f32>)),
    ("extrusion" => Extrusion(Vector4<f32>)),
}
