//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultiThreadCheck`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkMultiThreadCheck`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x11e4408b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultiThreadCheck {
    /// # C++ Class Fields Info
    /// -   name:`"threadId"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "threadId", skip_serializing)]
    ThreadId(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"stackTraceId"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "stackTraceId", skip_serializing)]
    StackTraceId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"markCount"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "markCount", skip_serializing)]
    MarkCount(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"markBitStack"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "markBitStack", skip_serializing)]
    MarkBitStack(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultiThreadCheck, "@name",
    ("threadId" => ThreadId(Primitive<u32>)),
    ("stackTraceId" => StackTraceId(Primitive<i32>)),
    ("markCount" => MarkCount(Primitive<u16>)),
    ("markBitStack" => MarkBitStack(Primitive<u16>)),
}

impl ByteDeSerialize for HkMultiThreadCheck {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AccessType {
    #[serde(rename = "HK_ACCESS_IGNORE")]
    HkAccessIgnore = 0,
    #[serde(rename = "HK_ACCESS_RO")]
    HkAccessRo = 1,
    #[serde(rename = "HK_ACCESS_RW")]
    HkAccessRw = 2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ReadMode {
    #[serde(rename = "THIS_OBJECT_ONLY")]
    ThisObjectOnly = 0,
    #[serde(rename = "RECURSIVE")]
    Recursive = 1,
}
