//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainMapperTarget`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpPoweredChainMapperTarget`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xf651c74d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainMapperTarget<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"chain"`
    /// -   type: `struct hkpPoweredChainData*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chain")]
    Chain(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"infoIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "infoIndex")]
    InfoIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperTarget<'de>, "@name",
    ("chain" => Chain(Primitive<Cow<'de, str>>)),
    ("infoIndex" => InfoIndex(Primitive<i32>)),
}

impl ByteDeSerialize for HkpPoweredChainMapperTarget<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
