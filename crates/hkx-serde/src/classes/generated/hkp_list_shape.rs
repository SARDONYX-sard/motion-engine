//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpListShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpListShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0xa1937cbd`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpListShape {
    /// # C++ Class Fields Info
    /// -   name:`"childInfo"`
    /// -   type: `hkArray&lt;struct hkpListShapeChildInfo&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childInfo")]
    ChildInfo(HkArrayClass<HkpListShapeChildInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkUint16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"numDisabledChildren"`
    /// -   type: `hkUint16`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numDisabledChildren")]
    NumDisabledChildren(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enabledChildren"`
    /// -   type: `hkUint32[8]`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enabledChildren")]
    EnabledChildren([Primitive<u32>; 8]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpListShape, "@name",
    ("childInfo" => ChildInfo(HkArrayClass<HkpListShapeChildInfo>)),
    ("flags" => Flags(Primitive<u16>)),
    ("numDisabledChildren" => NumDisabledChildren(Primitive<u16>)),
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("enabledChildren" => EnabledChildren([Primitive<u32>; 8])),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListShapeFlags {
    #[serde(rename = "ALL_FLAGS_CLEAR")]
    AllFlagsClear = 0,
    #[serde(rename = "DISABLE_SPU_CACHE_FOR_LIST_CHILD_INFO")]
    DisableSpuCacheForListChildInfo = 1,
}
