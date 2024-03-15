//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShapeChunk`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCompressedMeshShapeChunk`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x5d0d67bd`
/// -   version: 4
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShapeChunk {
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"indices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices")]
    Indices(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"stripLengths"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stripLengths")]
    StripLengths(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"materialInfo"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialInfo")]
    MaterialInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"reference"`
    /// -   type: `hkUint16`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reference")]
    Reference(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeChunk, "@name",
    ("offset" => Offset(Vector4<f32>)),
    ("vertices" => Vertices(HkArrayRef<Primitive<u16>>)),
    ("indices" => Indices(HkArrayRef<Primitive<u16>>)),
    ("stripLengths" => StripLengths(HkArrayRef<Primitive<u16>>)),
    ("weldingInfo" => WeldingInfo(HkArrayRef<Primitive<u16>>)),
    ("materialInfo" => MaterialInfo(Primitive<u32>)),
    ("reference" => Reference(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
