//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryMeshTexture`
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
    // `hkMeshTexture`(Parent class) has no fields

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

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
    Format(Primitive<Format>),
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
    FilterMode(Primitive<FilterMode>),
    /// # C++ Class Fields Info
    /// -   name:`"usageHint"`
    /// -   type: `enum TextureUsageType`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usageHint")]
    UsageHint(Primitive<TextureUsageType>),
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
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("filename" => Filename(Primitive<Cow<'de, str>>)),
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
    ("format" => Format(Primitive<Format>)),
    ("hasMipMaps" => HasMipMaps(Primitive<bool>)),
    ("filterMode" => FilterMode(Primitive<FilterMode>)),
    ("usageHint" => UsageHint(Primitive<TextureUsageType>)),
    ("textureCoordChannel" => TextureCoordChannel(Primitive<i32>)),
}
