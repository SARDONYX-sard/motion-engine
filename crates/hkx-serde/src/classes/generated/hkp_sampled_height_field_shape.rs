//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSampledHeightFieldShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSampledHeightFieldShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpHeightFieldShape`/`0xe7eca7eb`
/// - signature: `0x11213421`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSampledHeightFieldShape {
    /// # C++ Class Fields Info
    /// -   name:`"xRes"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xRes")]
    XRes(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"zRes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "zRes")]
    ZRes(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"heightCenter"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightCenter")]
    HeightCenter(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"useProjectionBasedHeight"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useProjectionBasedHeight")]
    UseProjectionBasedHeight(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"heightfieldType"`
    /// -   type: `enum HeightFieldType`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightfieldType")]
    HeightfieldType(HeightFieldType),
    /// # C++ Class Fields Info
    /// -   name:`"intToFloatScale"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "intToFloatScale")]
    IntToFloatScale(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatToIntScale"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatToIntScale")]
    FloatToIntScale(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"floatToIntOffsetFloorCorrected"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatToIntOffsetFloorCorrected")]
    FloatToIntOffsetFloorCorrected(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extents"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extents")]
    Extents(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSampledHeightFieldShape, "@name",
    ("xRes" => XRes(Primitive<i32>)),
    ("zRes" => ZRes(Primitive<i32>)),
    ("heightCenter" => HeightCenter(Primitive<f32>)),
    ("useProjectionBasedHeight" => UseProjectionBasedHeight(Primitive<bool>)),
    ("heightfieldType" => HeightfieldType(HeightFieldType)),
    ("intToFloatScale" => IntToFloatScale(Vector4<f32>)),
    ("floatToIntScale" => FloatToIntScale(Vector4<f32>)),
    ("floatToIntOffsetFloorCorrected" => FloatToIntOffsetFloorCorrected(Vector4<f32>)),
    ("extents" => Extents(Vector4<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeightFieldType {
    #[serde(rename = "HEIGHTFIELD_STORAGE")]
    HeightfieldStorage = 0,
    #[serde(rename = "HEIGHTFIELD_COMPRESSED")]
    HeightfieldCompressed = 1,
    #[serde(rename = "HEIGHTFIELD_USER")]
    HeightfieldUser = 2,
    #[serde(rename = "HEIGHTFIELD_MAX_ID")]
    HeightfieldMaxId = 3,
}
