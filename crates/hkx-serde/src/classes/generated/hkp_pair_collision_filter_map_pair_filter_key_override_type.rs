//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPairCollisionFilterMapPairFilterKeyOverrideType`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpPairCollisionFilterMapPairFilterKeyOverrideType`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x36195969`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"elem"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "elem", skip_serializing)]
    Elem(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"numElems"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElems")]
    NumElems(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"hashMod"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hashMod")]
    HashMod(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPairCollisionFilterMapPairFilterKeyOverrideType<'de>, "@name",
    ("elem" => Elem(Primitive<Cow<'de, str>>)),
    ("numElems" => NumElems(Primitive<i32>)),
    ("hashMod" => HashMod(Primitive<i32>)),
}
