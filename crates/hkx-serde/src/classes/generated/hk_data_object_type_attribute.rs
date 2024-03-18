//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkDataObjectTypeAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkDataObjectTypeAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0x1e3857bb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkDataObjectTypeAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"typeName"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "typeName")]
    TypeName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkDataObjectTypeAttribute<'de>, "@name",
    ("typeName" => TypeName(Primitive<Cow<'de, str>>)),
}
