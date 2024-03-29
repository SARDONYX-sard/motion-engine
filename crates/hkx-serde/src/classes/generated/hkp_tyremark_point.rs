//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTyremarkPoint`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpTyremarkPoint`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x6bb7c5e8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTyremarkPoint {
    /// # C++ Class Fields Info
    /// -   name:`"pointLeft"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointLeft")]
    PointLeft(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"pointRight"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointRight")]
    PointRight(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarkPoint, "@name",
    ("pointLeft" => PointLeft(Primitive<Vector4<f32>>)),
    ("pointRight" => PointRight(Primitive<Vector4<f32>>)),
}

impl ByteDeSerialize for HkpTyremarkPoint {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
