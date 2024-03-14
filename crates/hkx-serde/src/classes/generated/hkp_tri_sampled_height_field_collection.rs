//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpTriSampledHeightFieldCollection`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTriSampledHeightFieldCollection`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0xc291ddde`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriSampledHeightFieldCollection<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"heightfield"`
    /// -   type: `struct hkpSampledHeightFieldShape*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightfield")]
    Heightfield(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"triangleExtrusion"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleExtrusion")]
    TriangleExtrusion(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldCollection<'de>, "@name",
    ("heightfield" => Heightfield(Cow<'de, str>)),
    ("childSize" => ChildSize(Primitive<i32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingInfo" => WeldingInfo(HkArrayRef<Primitive<u16>>)),
    ("triangleExtrusion" => TriangleExtrusion(Vector4<f32>)),
}
