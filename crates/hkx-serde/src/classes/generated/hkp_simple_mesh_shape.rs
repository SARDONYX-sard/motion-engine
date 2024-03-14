//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSimpleMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSimpleMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0x16b3c811`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleMeshShape {
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"triangles"`
    /// -   type: `hkArray&lt;struct hkpSimpleMeshShapeTriangle&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangles")]
    Triangles(HkArrayClass<HkpSimpleMeshShapeTriangle>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleMeshShape, "@name",
    ("vertices" => Vertices(HkArrayVector<Vector4<f32>>)),
    ("triangles" => Triangles(HkArrayClass<HkpSimpleMeshShapeTriangle>)),
    ("materialIndices" => MaterialIndices(HkArrayRef<Primitive<u8>>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingType" => WeldingType(WeldingType)),
}
