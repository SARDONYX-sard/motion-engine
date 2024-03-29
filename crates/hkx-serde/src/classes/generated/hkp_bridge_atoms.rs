//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBridgeAtoms`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpBridgeAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0xde152a4d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBridgeAtoms<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"bridgeAtom"`
    /// -   type: `struct hkpBridgeConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bridgeAtom")]
    BridgeAtom(SingleClass<HkpBridgeConstraintAtom<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBridgeAtoms<'de>, "@name",
    ("bridgeAtom" => BridgeAtom(SingleClass<HkpBridgeConstraintAtom<'de>>)),
}

impl ByteDeSerialize for HkpBridgeAtoms<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
