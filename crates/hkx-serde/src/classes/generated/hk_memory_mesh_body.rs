//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshBody`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMemoryMeshBody`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkMeshBody`/`0xd0be5d7d`
/// - signature: `0x94a620a8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshBody<'a> {
    // C++ Parent class(`hkMeshBody` => parent: `hkReferencedObject`) has no fields

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
    /// -   name:`"transform"`
    /// -   type: `hkMatrix4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Matrix4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"transformSet"`
    /// -   type: `struct hkIndexedTransformSet*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformSet")]
    TransformSet(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkMeshShape*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexBuffers"`
    /// -   type: `hkArray<hkMeshVertexBuffer*>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffers")]
    VertexBuffers(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshBody<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("transform" => Transform(Matrix4<f32>)),
    ("transformSet" => TransformSet(Primitive<Cow<'de, str>>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("vertexBuffers" => VertexBuffers(HkArrayRef<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
