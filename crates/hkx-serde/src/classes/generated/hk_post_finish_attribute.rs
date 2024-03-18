//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPostFinishAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkPostFinishAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0x903abb2c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPostFinishAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"postFinishFunction"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "postFinishFunction", skip_serializing)]
    PostFinishFunction(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPostFinishAttribute<'de>, "@name",
    ("postFinishFunction" => PostFinishFunction(Primitive<Cow<'de, str>>)),
}
