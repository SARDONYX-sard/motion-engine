//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialShader`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxMaterialShader`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x28515eff`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialShader<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum ShaderType`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<ShaderType>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexEntryName")]
    VertexEntryName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"geomEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "geomEntryName")]
    GeomEntryName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"pixelEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pixelEntryName")]
    PixelEntryName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayRef<Primitive<u8>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialShader<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<ShaderType>)),
    ("vertexEntryName" => VertexEntryName(Primitive<Cow<'de, str>>)),
    ("geomEntryName" => GeomEntryName(Primitive<Cow<'de, str>>)),
    ("pixelEntryName" => PixelEntryName(Primitive<Cow<'de, str>>)),
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShaderType {
    #[serde(rename = "EFFECT_TYPE_INVALID")]
    EffectTypeInvalid = 0,
    #[serde(rename = "EFFECT_TYPE_UNKNOWN")]
    EffectTypeUnknown = 1,
    #[serde(rename = "EFFECT_TYPE_HLSL_INLINE")]
    EffectTypeHlslInline = 2,
    #[serde(rename = "EFFECT_TYPE_CG_INLINE")]
    EffectTypeCgInline = 3,
    #[serde(rename = "EFFECT_TYPE_HLSL_FILENAME")]
    EffectTypeHlslFilename = 4,
    #[serde(rename = "EFFECT_TYPE_CG_FILENAME")]
    EffectTypeCgFilename = 5,
    #[serde(rename = "EFFECT_TYPE_MAX_ID")]
    EffectTypeMaxId = 6,
}
