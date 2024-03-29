//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxAttributeGroup`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkxAttributeGroup`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x345ca95d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAttributeGroup<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"attributes"`
    /// -   type: `hkArray<struct hkxAttribute>`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributes")]
    Attributes(HkArrayClass<HkxAttribute<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAttributeGroup<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("attributes" => Attributes(HkArrayClass<HkxAttribute<'de>>)),
}

impl ByteDeSerialize for HkxAttributeGroup<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
