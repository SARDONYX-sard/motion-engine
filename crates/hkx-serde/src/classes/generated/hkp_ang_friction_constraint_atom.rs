//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAngFrictionConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpAngFrictionConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf313aa80`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngFrictionConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"firstFrictionAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "firstFrictionAxis")]
    FirstFrictionAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"numFrictionAxes"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numFrictionAxes")]
    NumFrictionAxes(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"maxFrictionTorque"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFrictionTorque")]
    MaxFrictionTorque(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngFrictionConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("firstFrictionAxis" => FirstFrictionAxis(Primitive<u8>)),
    ("numFrictionAxes" => NumFrictionAxes(Primitive<u8>)),
    ("maxFrictionTorque" => MaxFrictionTorque(Primitive<f32>)),
}

impl ByteDeSerialize for HkpAngFrictionConstraintAtom {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
