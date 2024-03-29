//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPulleyConstraintDataAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpPulleyConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0xb149e5a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPulleyConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"translations"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translations")]
    Translations(SingleClass<HkpSetLocalTranslationsConstraintAtom>),
    /// # C++ Class Fields Info
    /// -   name:`"pulley"`
    /// -   type: `struct hkpPulleyConstraintAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pulley")]
    Pulley(SingleClass<HkpPulleyConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintDataAtoms, "@name",
    ("translations" => Translations(SingleClass<HkpSetLocalTranslationsConstraintAtom>)),
    ("pulley" => Pulley(SingleClass<HkpPulleyConstraintAtom>)),
}

impl ByteDeSerialize for HkpPulleyConstraintDataAtoms {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
