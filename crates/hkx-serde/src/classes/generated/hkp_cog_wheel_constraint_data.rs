//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCogWheelConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpCogWheelConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x7f0e53fc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCogWheelConstraintData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpCogWheelConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpCogWheelConstraintDataAtoms>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCogWheelConstraintData, "@name",
    ("atoms" => Atoms(SingleClass<HkpCogWheelConstraintDataAtoms>)),
}

impl ByteDeSerialize for HkpCogWheelConstraintData {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
