//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinearParametricCurve`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpLinearParametricCurve`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpParametricCurve`/`0xda8c7d7d`
/// - signature: `0xd7b3be03`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinearParametricCurve {
    // C++ Parent class(`hkpParametricCurve` => parent: `hkReferencedObject`) has no fields

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"smoothingFactor"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "smoothingFactor")]
    SmoothingFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"closedLoop"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "closedLoop")]
    ClosedLoop(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"dirNotParallelToTangentAlongWholePath"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dirNotParallelToTangentAlongWholePath")]
    DirNotParallelToTangentAlongWholePath(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"points"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "points")]
    Points(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"distance"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distance")]
    Distance(HkArrayRef<Primitive<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinearParametricCurve, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("smoothingFactor" => SmoothingFactor(Primitive<f32>)),
    ("closedLoop" => ClosedLoop(Primitive<bool>)),
    ("dirNotParallelToTangentAlongWholePath" => DirNotParallelToTangentAlongWholePath(Vector4<f32>)),
    ("points" => Points(HkArrayVector<Vector4<f32>>)),
    ("distance" => Distance(HkArrayRef<Primitive<f32>>)),
}
