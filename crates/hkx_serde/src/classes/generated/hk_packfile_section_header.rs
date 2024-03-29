//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPackfileSectionHeader`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkPackfileSectionHeader`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xf2a92154`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkPackfileSectionHeader {
    /// # C++ Class Fields Info
    /// -   name:`"sectionTag"`
    /// -   type: `hkChar[19]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectionTag")]
    SectionTag(CStyleArray<[char; 19]>),
    /// # C++ Class Fields Info
    /// -   name:`"nullByte"`
    /// -   type: `hkChar`
    /// - offset: 19
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nullByte")]
    NullByte(Primitive<char>),
    /// # C++ Class Fields Info
    /// -   name:`"absoluteDataStart"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absoluteDataStart")]
    AbsoluteDataStart(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"localFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFixupsOffset")]
    LocalFixupsOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"globalFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "globalFixupsOffset")]
    GlobalFixupsOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"virtualFixupsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "virtualFixupsOffset")]
    VirtualFixupsOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"exportsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exportsOffset")]
    ExportsOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"importsOffset"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "importsOffset")]
    ImportsOffset(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"endOffset"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endOffset")]
    EndOffset(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPackfileSectionHeader, "@name",
    ("sectionTag" => SectionTag(CStyleArray<[char; 19]>)),
    ("nullByte" => NullByte(Primitive<char>)),
    ("absoluteDataStart" => AbsoluteDataStart(Primitive<i32>)),
    ("localFixupsOffset" => LocalFixupsOffset(Primitive<i32>)),
    ("globalFixupsOffset" => GlobalFixupsOffset(Primitive<i32>)),
    ("virtualFixupsOffset" => VirtualFixupsOffset(Primitive<i32>)),
    ("exportsOffset" => ExportsOffset(Primitive<i32>)),
    ("importsOffset" => ImportsOffset(Primitive<i32>)),
    ("endOffset" => EndOffset(Primitive<i32>)),
}

impl ByteDeSerialize for HkPackfileSectionHeader {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
