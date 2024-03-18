//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpNamedMeshMaterial`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpNamedMeshMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `hkpMeshMaterial`/`0x886cde0c`
/// - signature: `0x66b42df1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpNamedMeshMaterial<'a> {
    /// # C++ Parent class(`hkpMeshMaterial` => parent: `None`) field Info
    /// -   name:`"filterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "filterInfo")]
    FilterInfo(Primitive<u32>),

    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpNamedMeshMaterial<'de>, "@name",
    ("filterInfo" => FilterInfo(Primitive<u32>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
