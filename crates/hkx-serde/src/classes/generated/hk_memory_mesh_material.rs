//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshMaterial`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMemoryMeshMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkMeshMaterial`/`0xda8c7d7d`
/// - signature: `0x12156ee3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshMaterial<'a> {
    // C++ Parent class(`hkMeshMaterial` => parent: `hkReferencedObject`) has no fields

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
    /// -   name:`"materialName"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialName")]
    MaterialName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"textures"`
    /// -   type: `hkArray<hkMeshTexture*>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "textures")]
    Textures(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshMaterial<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("materialName" => MaterialName(Primitive<Cow<'de, str>>)),
    ("textures" => Textures(HkArrayRef<Cow<'de, str>>)),
}
