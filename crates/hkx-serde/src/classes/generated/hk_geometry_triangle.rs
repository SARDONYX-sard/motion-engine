//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkGeometryTriangle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkGeometryTriangle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x9687513b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkGeometryTriangle {
    /// # C++ Class Fields Info
    /// -   name:`"a"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "a")]
    A(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"b"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "b")]
    B(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"c"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "c")]
    C(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkGeometryTriangle, "@name",
    ("a" => A(Primitive<i32>)),
    ("b" => B(Primitive<i32>)),
    ("c" => C(Primitive<i32>)),
    ("material" => Material(Primitive<i32>)),
}

impl ByteDeSerialize for HkGeometryTriangle {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
