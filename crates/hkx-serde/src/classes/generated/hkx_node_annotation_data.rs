//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxNodeAnnotationData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxNodeAnnotationData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x433dee92`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxNodeAnnotationData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"description"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "description")]
    Description(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxNodeAnnotationData<'de>, "@name",
    ("time" => Time(Primitive<f32>)),
    ("description" => Description(Primitive<Cow<'de, str>>)),
}
