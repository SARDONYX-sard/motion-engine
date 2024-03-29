//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialProperty`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkxMaterialProperty`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xd295234d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialProperty {
    /// # C++ Class Fields Info
    /// -   name:`"key"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "key")]
    Key(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialProperty, "@name",
    ("key" => Key(Primitive<u32>)),
    ("value" => Value(Primitive<u32>)),
}

impl ByteDeSerialize for HkxMaterialProperty {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
