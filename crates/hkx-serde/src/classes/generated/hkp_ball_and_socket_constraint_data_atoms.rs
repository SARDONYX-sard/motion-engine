//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallAndSocketConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpBallAndSocketConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xc73dcaf9`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallAndSocketConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"pivots"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivots")]
    Pivots(SingleClass<HkpSetLocalTranslationsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setupStabilization")]
    SetupStabilization(SingleClass<HkpSetupStabilizationAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallAndSocketConstraintDataAtoms, "@name",
    ("pivots" => Pivots(SingleClass<HkpSetLocalTranslationsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

impl ByteDeSerialize for HkpBallAndSocketConstraintDataAtoms {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
