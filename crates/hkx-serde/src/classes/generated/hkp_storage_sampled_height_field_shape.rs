//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageSampledHeightFieldShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpStorageSampledHeightFieldShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpSampledHeightFieldShape`/`0x11213421`
/// - signature: `0x15ff414b`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageSampledHeightFieldShape {
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"xRes"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "xRes", default)]
    XRes(Primitive<i32>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"zRes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "zRes", default)]
    ZRes(Primitive<i32>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"heightCenter"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightCenter", default)]
    HeightCenter(Primitive<f32>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"useProjectionBasedHeight"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useProjectionBasedHeight", default)]
    UseProjectionBasedHeight(Primitive<bool>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"heightfieldType"`
    /// -   type: `enum HeightFieldType`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightfieldType", default)]
    HeightfieldType(Primitive<HeightFieldType>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"intToFloatScale"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "intToFloatScale", default)]
    IntToFloatScale(Vector4<f32>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"floatToIntScale"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatToIntScale", default)]
    FloatToIntScale(Vector4<f32>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"floatToIntOffsetFloorCorrected"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatToIntOffsetFloorCorrected", default)]
    FloatToIntOffsetFloorCorrected(Vector4<f32>),
    /// # C++ Parent class(`hkpSampledHeightFieldShape`, parent: `hkpHeightFieldShape`) field Info
    /// -   name:`"extents"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extents", default)]
    Extents(Vector4<f32>),

    // `hkpHeightFieldShape`(Parent class) has no fields

    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", default, skip_serializing)]
    Type(Primitive<Unknown>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"storage"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "storage", default)]
    Storage(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"triangleFlip"`
    /// -   type: `hkBool`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleFlip", default)]
    TriangleFlip(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageSampledHeightFieldShape, "@name",
    ("xRes" => XRes(Primitive<i32>)),
    ("zRes" => ZRes(Primitive<i32>)),
    ("heightCenter" => HeightCenter(Primitive<f32>)),
    ("useProjectionBasedHeight" => UseProjectionBasedHeight(Primitive<bool>)),
    ("heightfieldType" => HeightfieldType(Primitive<HeightFieldType>)),
    ("intToFloatScale" => IntToFloatScale(Vector4<f32>)),
    ("floatToIntScale" => FloatToIntScale(Vector4<f32>)),
    ("floatToIntOffsetFloorCorrected" => FloatToIntOffsetFloorCorrected(Vector4<f32>)),
    ("extents" => Extents(Vector4<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<Unknown>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("storage" => Storage(HkArrayRef<Primitive<f32>>)),
    ("triangleFlip" => TriangleFlip(Primitive<bool>)),
}
