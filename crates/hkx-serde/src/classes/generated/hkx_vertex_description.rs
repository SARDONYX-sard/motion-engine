//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexDescription`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxVertexDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x2df6313d`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexDescription {
    /// # C++ Class Fields Info
    /// -   name:`"decls"`
    /// -   type: `hkArray&lt;struct hkxVertexDescriptionElementDecl&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "decls")]
    Decls(HkArrayClass<HkxVertexDescriptionElementDecl>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescription, "@name",
    ("decls" => Decls(HkArrayClass<HkxVertexDescriptionElementDecl>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "HKX_DT_NONE")]
    HkxDtNone = 0,
    #[serde(rename = "HKX_DT_UINT8")]
    HkxDtUint8 = 1,
    #[serde(rename = "HKX_DT_INT16")]
    HkxDtInt16 = 2,
    #[serde(rename = "HKX_DT_UINT32")]
    HkxDtUint32 = 3,
    #[serde(rename = "HKX_DT_FLOAT")]
    HkxDtFloat = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataUsage {
    #[serde(rename = "HKX_DU_NONE")]
    HkxDuNone = 0,
    #[serde(rename = "HKX_DU_POSITION")]
    HkxDuPosition = 1,
    #[serde(rename = "HKX_DU_COLOR")]
    HkxDuColor = 2,
    #[serde(rename = "HKX_DU_NORMAL")]
    HkxDuNormal = 4,
    #[serde(rename = "HKX_DU_TANGENT")]
    HkxDuTangent = 8,
    #[serde(rename = "HKX_DU_BINORMAL")]
    HkxDuBinormal = 16,
    #[serde(rename = "HKX_DU_TEXCOORD")]
    HkxDuTexcoord = 32,
    #[serde(rename = "HKX_DU_BLENDWEIGHTS")]
    HkxDuBlendweights = 64,
    #[serde(rename = "HKX_DU_BLENDINDICES")]
    HkxDuBlendindices = 128,
    #[serde(rename = "HKX_DU_USERDATA")]
    HkxDuUserdata = 256,
}
