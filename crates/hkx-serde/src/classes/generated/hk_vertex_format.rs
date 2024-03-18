//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkVertexFormat`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkVertexFormat`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 260
/// -    vtable: false
/// - signature: `0xf11e3ff7`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkVertexFormat {
    /// # C++ Class Fields Info
    /// -   name:`"elements"`
    /// -   type: `struct hkVertexFormatElement[32]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elements")]
    Elements(CStyleArrayClass<HkVertexFormatElement, 32>),
    /// # C++ Class Fields Info
    /// -   name:`"numElements"`
    /// -   type: `hkInt32`
    /// - offset: 256
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElements")]
    NumElements(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkVertexFormat, "@name",
    ("elements" => Elements(CStyleArrayClass<HkVertexFormatElement, 32>)),
    ("numElements" => NumElements(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "TYPE_NONE")]
    TypeNone = 0,
    #[serde(rename = "TYPE_INT8")]
    TypeInt8 = 1,
    #[serde(rename = "TYPE_UINT8")]
    TypeUint8 = 2,
    #[serde(rename = "TYPE_INT16")]
    TypeInt16 = 3,
    #[serde(rename = "TYPE_UINT16")]
    TypeUint16 = 4,
    #[serde(rename = "TYPE_INT32")]
    TypeInt32 = 5,
    #[serde(rename = "TYPE_UINT32")]
    TypeUint32 = 6,
    #[serde(rename = "TYPE_UINT8_DWORD")]
    TypeUint8Dword = 7,
    #[serde(rename = "TYPE_ARGB32")]
    TypeArgb32 = 8,
    #[serde(rename = "TYPE_FLOAT16")]
    TypeFloat16 = 9,
    #[serde(rename = "TYPE_FLOAT32")]
    TypeFloat32 = 10,
    #[serde(rename = "TYPE_VECTOR4")]
    TypeVector4 = 11,
    #[serde(rename = "TYPE_LAST")]
    TypeLast = 12,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentUsage {
    #[serde(rename = "USAGE_NONE")]
    UsageNone = 0,
    #[serde(rename = "USAGE_POSITION")]
    UsagePosition = 1,
    #[serde(rename = "USAGE_NORMAL")]
    UsageNormal = 2,
    #[serde(rename = "USAGE_COLOR")]
    UsageColor = 3,
    #[serde(rename = "USAGE_TANGENT")]
    UsageTangent = 4,
    #[serde(rename = "USAGE_BINORMAL")]
    UsageBinormal = 5,
    #[serde(rename = "USAGE_BLEND_MATRIX_INDEX")]
    UsageBlendMatrixIndex = 6,
    #[serde(rename = "USAGE_BLEND_WEIGHTS")]
    UsageBlendWeights = 7,
    #[serde(rename = "USAGE_BLEND_WEIGHTS_LAST_IMPLIED")]
    UsageBlendWeightsLastImplied = 8,
    #[serde(rename = "USAGE_TEX_COORD")]
    UsageTexCoord = 9,
    #[serde(rename = "USAGE_POINT_SIZE")]
    UsagePointSize = 10,
    #[serde(rename = "USAGE_USER")]
    UsageUser = 11,
    #[serde(rename = "USAGE_LAST")]
    UsageLast = 12,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HintFlags {
    #[serde(rename = "FLAG_READ")]
    FlagRead = 1,
    #[serde(rename = "FLAG_WRITE")]
    FlagWrite = 2,
    #[serde(rename = "FLAG_DYNAMIC")]
    FlagDynamic = 4,
    #[serde(rename = "FLAG_NOT_SHARED")]
    FlagNotShared = 8,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SharingType {
    #[serde(rename = "SHARING_ALL_SHARED")]
    SharingAllShared = 0,
    #[serde(rename = "SHARING_ALL_NOT_SHARED")]
    SharingAllNotShared = 1,
    #[serde(rename = "SHARING_MIXTURE")]
    SharingMixture = 2,
}
