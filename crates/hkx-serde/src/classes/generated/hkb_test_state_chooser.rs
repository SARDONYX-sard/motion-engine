//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTestStateChooser`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbTestStateChooser`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkbStateChooser`/`0xda8c7d7d`
/// - signature: `0xc0fcc436`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTestStateChooser<'a> {
    // C++ Parent class(`hkbStateChooser` => parent: `hkReferencedObject`) has no fields

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
    /// -   name:`"int"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "int")]
    Int(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"real"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "real")]
    Real(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"string"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "string")]
    String(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTestStateChooser<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("int" => Int(Primitive<i32>)),
    ("real" => Real(Primitive<f32>)),
    ("string" => String(Primitive<Cow<'de, str>>)),
}
