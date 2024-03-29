//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRootLevelContainerNamedVariant`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkRootLevelContainerNamedVariant`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xb103a2cd`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkRootLevelContainerNamedVariant<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"className"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "className")]
    ClassName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"variant"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variant")]
    Variant(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRootLevelContainerNamedVariant<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("className" => ClassName(Primitive<Cow<'de, str>>)),
    ("variant" => Variant(Primitive<Cow<'de, str>>)),
}
