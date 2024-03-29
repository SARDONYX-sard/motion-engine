//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkVertexFormatElement`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkVertexFormatElement`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x54867cbf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkVertexFormatElement {
    /// # C++ Class Fields Info
    /// -   name:`"dataType"`
    /// -   type: `enum ComponentType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dataType")]
    DataType(Primitive<ComponentType>),
    /// # C++ Class Fields Info
    /// -   name:`"numValues"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numValues")]
    NumValues(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"usage"`
    /// -   type: `enum ComponentUsage`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usage")]
    Usage(Primitive<ComponentUsage>),
    /// # C++ Class Fields Info
    /// -   name:`"subUsage"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subUsage")]
    SubUsage(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags HintFlags`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<HintFlags>),
    /// # C++ Class Fields Info
    /// -   name:`"pad"`
    /// -   type: `hkUint8[3]`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad")]
    Pad(CStyleArray<[u8; 3]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkVertexFormatElement, "@name",
    ("dataType" => DataType(Primitive<ComponentType>)),
    ("numValues" => NumValues(Primitive<u8>)),
    ("usage" => Usage(Primitive<ComponentUsage>)),
    ("subUsage" => SubUsage(Primitive<u8>)),
    ("flags" => Flags(Primitive<HintFlags>)),
    ("pad" => Pad(CStyleArray<[u8; 3]>)),
}

impl ByteDeSerialize for HkVertexFormatElement {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
