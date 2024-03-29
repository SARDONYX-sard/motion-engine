//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRangeInt32Attribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkRangeInt32Attribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x4846be29`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkRangeInt32Attribute {
    /// # C++ Class Fields Info
    /// -   name:`"absmin"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absmin")]
    Absmin(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"absmax"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absmax")]
    Absmax(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"softmin"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "softmin")]
    Softmin(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"softmax"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "softmax")]
    Softmax(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRangeInt32Attribute, "@name",
    ("absmin" => Absmin(Primitive<i32>)),
    ("absmax" => Absmax(Primitive<i32>)),
    ("softmin" => Softmin(Primitive<i32>)),
    ("softmax" => Softmax(Primitive<i32>)),
}

impl ByteDeSerialize for HkRangeInt32Attribute {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
