//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleContactConstraintDataInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpSimpleContactConstraintDataInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xb59d1734`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleContactConstraintDataInfo {
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "flags")]
    Flags(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"index"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "index")]
    Index(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"internalData0"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalData0")]
    InternalData0(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rollingFrictionMultiplier"`
    /// -   type: `hkHalf`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rollingFrictionMultiplier")]
    RollingFrictionMultiplier(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"internalData1"`
    /// -   type: `hkHalf`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalData1")]
    InternalData1(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkUint32[5]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(CStyleArray<[u32; 5]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleContactConstraintDataInfo, "@name",
    ("flags" => Flags(Primitive<u16>)),
    ("index" => Index(Primitive<u16>)),
    ("internalData0" => InternalData0(Primitive<f32>)),
    ("rollingFrictionMultiplier" => RollingFrictionMultiplier(Primitive<f32>)),
    ("internalData1" => InternalData1(Primitive<f32>)),
    ("data" => Data(CStyleArray<[u32; 5]>)),
}
