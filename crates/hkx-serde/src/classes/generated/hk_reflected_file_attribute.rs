//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkReflectedFileAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkReflectedFileAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0xedb6b8f7`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkReflectedFileAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `char*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkReflectedFileAttribute<'de>, "@name",
    ("value" => Value(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkReflectedFileAttribute<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
