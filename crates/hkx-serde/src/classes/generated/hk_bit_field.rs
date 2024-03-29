//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkBitField`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkBitField`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xda41bd9b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkBitField {
    /// # C++ Class Fields Info
    /// -   name:`"words"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "words")]
    Words(HkArrayNum<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"numBits"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBits")]
    NumBits(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkBitField, "@name",
    ("words" => Words(HkArrayNum<u32>)),
    ("numBits" => NumBits(Primitive<i32>)),
}

impl ByteDeSerialize for HkBitField {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
