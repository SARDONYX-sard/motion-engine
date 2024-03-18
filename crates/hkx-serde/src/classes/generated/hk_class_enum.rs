//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkClassEnum`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkClassEnum`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0x8a3609cf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkClassEnum<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"items"`
    /// -   type: `hkSimpleArray<struct hkClassEnumItem>`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "items")]
    Items(HkArrayClass<HkClassEnumItem<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"attributes"`
    /// -   type: `struct hkCustomAttributes*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "attributes", skip_serializing)]
    Attributes(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagValues`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<FlagValues>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkClassEnum<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("items" => Items(HkArrayClass<HkClassEnumItem<'de>>)),
    ("attributes" => Attributes(Primitive<Cow<'de, str>>)),
    ("flags" => Flags(Primitive<FlagValues>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlagValues {
    #[serde(rename = "FLAGS_NONE")]
    FlagsNone = 0,
}
