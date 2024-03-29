//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMalleableConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpMalleableConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x6748b2cf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMalleableConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraintData")]
    ConstraintData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMalleableConstraintData<'de>, "@name",
    ("constraintData" => ConstraintData(Primitive<Cow<'de, str>>)),
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("strength" => Strength(Primitive<f32>)),
}

impl ByteDeSerialize for HkpMalleableConstraintData<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
