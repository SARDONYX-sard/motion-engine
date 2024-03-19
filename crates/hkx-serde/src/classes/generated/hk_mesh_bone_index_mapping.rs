//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshBoneIndexMapping`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMeshBoneIndexMapping`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x48aceb75`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshBoneIndexMapping {
    /// # C++ Class Fields Info
    /// -   name:`"mapping"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mapping")]
    Mapping(HkArrayNum<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshBoneIndexMapping, "@name",
    ("mapping" => Mapping(HkArrayNum<i16>)),
}
