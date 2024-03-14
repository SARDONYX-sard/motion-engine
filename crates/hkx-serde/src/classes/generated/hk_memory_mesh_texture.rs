//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMemoryMeshTexture`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMemoryMeshTexture`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkMeshTexture`/`0xc9887918`
/// - signature: `0x2db6577c`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshTexture<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"filename"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "filename")]
    Filename(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"format"`
    /// -   type: `enum Format`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "format")]
    Format(Format),
    /// # C++ Class Fields Info
    /// -   name:`"hasMipMaps"`
    /// -   type: `hkBool`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hasMipMaps")]
    HasMipMaps(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"filterMode"`
    /// -   type: `enum FilterMode`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "filterMode")]
    FilterMode(FilterMode),
    /// # C++ Class Fields Info
    /// -   name:`"usageHint"`
    /// -   type: `enum TextureUsageType`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usageHint")]
    UsageHint(TextureUsageType),
    /// # C++ Class Fields Info
    /// -   name:`"textureCoordChannel"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "textureCoordChannel")]
    TextureCoordChannel(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshTexture<'de>, "@name",
    ("filename" => Filename(Primitive<Cow<'de, str>>)),
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
    ("format" => Format(Format)),
    ("hasMipMaps" => HasMipMaps(Primitive<bool>)),
    ("filterMode" => FilterMode(FilterMode)),
    ("usageHint" => UsageHint(TextureUsageType)),
    ("textureCoordChannel" => TextureCoordChannel(Primitive<i32>)),
}
