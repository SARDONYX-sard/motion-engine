//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollLimitsData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpRagdollLimitsData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 192
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0xcbdb44aa`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollLimitsData {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpRagdollLimitsDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpRagdollLimitsDataAtoms>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollLimitsData, "@name",
    ("atoms" => Atoms(SingleClass<HkpRagdollLimitsDataAtoms>)),
}

impl ByteDeSerialize for HkpRagdollLimitsData {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
