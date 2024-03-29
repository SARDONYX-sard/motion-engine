//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpRagdollConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 352
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x8fb5dd29`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpRagdollConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpRagdollConstraintDataAtoms<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollConstraintData<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpRagdollConstraintDataAtoms<'de>>)),
}

impl ByteDeSerialize for HkpRagdollConstraintData<'_> {
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
pub enum MotorIndex {
    #[serde(rename = "MOTOR_TWIST")]
    MotorTwist = 0,
    #[serde(rename = "MOTOR_PLANE")]
    MotorPlane = 1,
    #[serde(rename = "MOTOR_CONE")]
    MotorCone = 2,
}
