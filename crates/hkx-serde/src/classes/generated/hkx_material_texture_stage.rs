//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialTextureStage`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxMaterialTextureStage`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xfa6facb2`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialTextureStage<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"texture"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "texture")]
    Texture(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"usageHint"`
    /// -   type: `enum TextureType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usageHint")]
    UsageHint(Primitive<TextureType>),
    /// # C++ Class Fields Info
    /// -   name:`"tcoordChannel"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tcoordChannel")]
    TcoordChannel(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialTextureStage<'de>, "@name",
    ("texture" => Texture(Primitive<Cow<'de, str>>)),
    ("usageHint" => UsageHint(Primitive<TextureType>)),
    ("tcoordChannel" => TcoordChannel(Primitive<i32>)),
}
