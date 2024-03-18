//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryResourceContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMemoryResourceContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkResourceContainer`/`0x4e94146`
/// - signature: `0x4762f92a`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryResourceContainer<'a> {
    // C++ Parent class(`hkResourceContainer` => parent: `hkResourceBase`) has no fields

    // C++ Parent class(`hkResourceBase` => parent: `hkReferencedObject`) has no fields

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
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"parent"`
    /// -   type: `struct hkMemoryResourceContainer*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "parent", skip_serializing)]
    Parent(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"resourceHandles"`
    /// -   type: `hkArray<hkMemoryResourceHandle*>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resourceHandles")]
    ResourceHandles(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkMemoryResourceContainer*>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceContainer<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
    ("resourceHandles" => ResourceHandles(HkArrayRef<Cow<'de, str>>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
}
