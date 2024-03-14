//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshTexture`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMeshTexture`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc9887918`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshTexture {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshTexture, "@name",
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "Unknown")]
    Unknown = 0,
    #[serde(rename = "PNG")]
    Png = 1,
    #[serde(rename = "TGA")]
    Tga = 2,
    #[serde(rename = "BMP")]
    Bmp = 3,
    #[serde(rename = "DDS")]
    Dds = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterMode {
    #[serde(rename = "POINT")]
    Point = 0,
    #[serde(rename = "LINEAR")]
    Linear = 1,
    #[serde(rename = "ANISOTROPIC")]
    Anisotropic = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextureUsageType {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "DIFFUSE")]
    Diffuse = 1,
    #[serde(rename = "REFLECTION")]
    Reflection = 2,
    #[serde(rename = "BUMP")]
    Bump = 3,
    #[serde(rename = "NORMAL")]
    Normal = 4,
    #[serde(rename = "DISPLACEMENT")]
    Displacement = 5,
    #[serde(rename = "SPECULAR")]
    Specular = 6,
    #[serde(rename = "SPECULARANDGLOSS")]
    Specularandgloss = 7,
    #[serde(rename = "OPACITY")]
    Opacity = 8,
    #[serde(rename = "EMISSIVE")]
    Emissive = 9,
    #[serde(rename = "REFRACTION")]
    Refraction = 10,
    #[serde(rename = "GLOSS")]
    Gloss = 11,
    #[serde(rename = "NOTEXPORTED")]
    Notexported = 12,
}
