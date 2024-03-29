//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkCustomAttributesAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkCustomAttributesAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x1388d601`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkCustomAttributesAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `hkVariant`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Primitive<u64>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkCustomAttributesAttribute<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("value" => Value(Primitive<u64>)),
}

impl ByteDeSerialize for HkCustomAttributesAttribute<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
