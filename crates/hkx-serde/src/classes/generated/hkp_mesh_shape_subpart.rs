//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMeshShapeSubpart`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpMeshShapeSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: false
/// - signature: `0x27336e5d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMeshShapeSubpart<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBase"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "vertexBase", skip_serializing)]
    VertexBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStriding")]
    VertexStriding(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "indexBase", skip_serializing)]
    IndexBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"stridingType"`
    /// -   type: `enum MeshShapeIndexStridingType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stridingType")]
    StridingType(Primitive<MeshShapeIndexStridingType>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MeshShapeMaterialIndexStridingType`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MeshShapeMaterialIndexStridingType>),
    /// # C++ Class Fields Info
    /// -   name:`"indexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexStriding")]
    IndexStriding(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"flipAlternateTriangles"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flipAlternateTriangles")]
    FlipAlternateTriangles(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numTriangles"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numTriangles")]
    NumTriangles(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialStriding")]
    MaterialStriding(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"triangleOffset"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleOffset")]
    TriangleOffset(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMeshShapeSubpart<'de>, "@name",
    ("vertexBase" => VertexBase(Primitive<Cow<'de, str>>)),
    ("vertexStriding" => VertexStriding(Primitive<i32>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("indexBase" => IndexBase(Primitive<Cow<'de, str>>)),
    ("stridingType" => StridingType(Primitive<MeshShapeIndexStridingType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MeshShapeMaterialIndexStridingType>)),
    ("indexStriding" => IndexStriding(Primitive<i32>)),
    ("flipAlternateTriangles" => FlipAlternateTriangles(Primitive<i32>)),
    ("numTriangles" => NumTriangles(Primitive<i32>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<i32>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("materialStriding" => MaterialStriding(Primitive<i32>)),
    ("numMaterials" => NumMaterials(Primitive<i32>)),
    ("triangleOffset" => TriangleOffset(Primitive<i32>)),
}
