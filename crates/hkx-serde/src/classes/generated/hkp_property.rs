//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpProperty`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpProperty`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x9ce308e9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpProperty {
    /// # C++ Class Fields Info
    /// -   name:`"key"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "key")]
    Key(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"alignmentPadding"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignmentPadding")]
    AlignmentPadding(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `struct hkpPropertyValue`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(SingleClass<HkpPropertyValue>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpProperty, "@name",
    ("key" => Key(Primitive<u32>)),
    ("alignmentPadding" => AlignmentPadding(Primitive<u32>)),
    ("value" => Value(SingleClass<HkpPropertyValue>)),
}
