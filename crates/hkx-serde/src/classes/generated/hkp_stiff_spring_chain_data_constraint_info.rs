//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStiffSpringChainDataConstraintInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpStiffSpringChainDataConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xc624a180`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStiffSpringChainDataConstraintInfo {
    /// # C++ Class Fields Info
    /// -   name:`"pivotInA"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInA")]
    PivotInA(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"pivotInB"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInB")]
    PivotInB(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"springLength"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springLength")]
    SpringLength(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStiffSpringChainDataConstraintInfo, "@name",
    ("pivotInA" => PivotInA(Primitive<Vector4<f32>>)),
    ("pivotInB" => PivotInB(Primitive<Vector4<f32>>)),
    ("springLength" => SpringLength(Primitive<f32>)),
}

impl ByteDeSerialize for HkpStiffSpringChainDataConstraintInfo {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
