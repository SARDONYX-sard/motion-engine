//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaDeltaCompressedAnimationQuantizationFormat`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaDeltaCompressedAnimationQuantizationFormat`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0x724a7561`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaDeltaCompressedAnimationQuantizationFormat {
    /// # C++ Class Fields Info
    /// -   name:`"maxBitWidth"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxBitWidth")]
    MaxBitWidth(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"preserved"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "preserved")]
    Preserved(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"numD"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numD")]
    NumD(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"offsetIdx"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetIdx")]
    OffsetIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"scaleIdx"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleIdx")]
    ScaleIdx(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"bitWidthIdx"`
    /// -   type: `hkUint32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitWidthIdx")]
    BitWidthIdx(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaDeltaCompressedAnimationQuantizationFormat, "@name",
    ("maxBitWidth" => MaxBitWidth(Primitive<u8>)),
    ("preserved" => Preserved(Primitive<u8>)),
    ("numD" => NumD(Primitive<u32>)),
    ("offsetIdx" => OffsetIdx(Primitive<u32>)),
    ("scaleIdx" => ScaleIdx(Primitive<u32>)),
    ("bitWidthIdx" => BitWidthIdx(Primitive<u32>)),
}
