//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSingleShapeContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpSingleShapeContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: true
/// -    parent: `hkpShapeContainer`/`0xe0708a00`
/// - signature: `0x73aa1d38`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSingleShapeContainer<'a> {
    // C++ Parent class(`hkpShapeContainer` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape")]
    ChildShape(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSingleShapeContainer<'de>, "@name",
    ("childShape" => ChildShape(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkpSingleShapeContainer<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
