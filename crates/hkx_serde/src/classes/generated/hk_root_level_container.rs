//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRootLevelContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use self::packfile_deserializer::PackFileDeserializer;

#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkRootLevelContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x2772c11e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkRootLevelContainer<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"namedVariants"`
    /// -   type: `hkArray<struct hkRootLevelContainerNamedVariant>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedVariants")]
    NamedVariants(HkArrayClass<HkRootLevelContainerNamedVariant<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRootLevelContainer<'de>, "@name",
    ("namedVariants" => NamedVariants(HkArrayClass<HkRootLevelContainerNamedVariant<'de>>)),
}

impl ByteDeSerialize for HkRootLevelContainer<'_> {
    fn from_bytes<B>(bytes: &[u8], de: &mut PackFileDeserializer) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
