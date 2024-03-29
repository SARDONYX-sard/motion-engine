//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkSemanticsAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkSemanticsAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x837099c3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSemanticsAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum Semantics`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<Semantics>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkSemanticsAttribute, "@name",
    ("type" => Type(Primitive<Semantics>)),
}

impl ByteDeSerialize for HkSemanticsAttribute {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Semantics {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "DISTANCE")]
    Distance = 1,
    #[serde(rename = "ANGLE")]
    Angle = 2,
    #[serde(rename = "NORMAL")]
    Normal = 3,
    #[serde(rename = "POSITION")]
    Position = 4,
    #[serde(rename = "COSINE_ANGLE")]
    CosineAngle = 5,
}
