//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMeshUserChannelInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxMeshUserChannelInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0x270724a5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMeshUserChannelInfo<'a> {
    /// # C++ Parent class(`hkxAttributeHolder` => parent: `hkReferencedObject`) field Info
    /// -   name:`"attributeGroups"`
    /// -   type: `hkArray<struct hkxAttributeGroup>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeGroups")]
    AttributeGroups(HkArrayClass<HkxAttributeGroup<'a>>),

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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"className"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "className")]
    ClassName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMeshUserChannelInfo<'de>, "@name",
    ("attributeGroups" => AttributeGroups(HkArrayClass<HkxAttributeGroup<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("className" => ClassName(Primitive<Cow<'de, str>>)),
}
