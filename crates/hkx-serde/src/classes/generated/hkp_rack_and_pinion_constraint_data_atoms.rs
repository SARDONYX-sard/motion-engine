//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRackAndPinionConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpRackAndPinionConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: false
/// - signature: `0xa58a9659`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRackAndPinionConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"rackAndPinion"`
    /// -   type: `struct hkpRackAndPinionConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rackAndPinion")]
    RackAndPinion(SingleClass<HkpRackAndPinionConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRackAndPinionConstraintDataAtoms, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("rackAndPinion" => RackAndPinion(SingleClass<HkpRackAndPinionConstraintAtom>)),
}

impl ByteDeSerialize for HkpRackAndPinionConstraintDataAtoms {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
