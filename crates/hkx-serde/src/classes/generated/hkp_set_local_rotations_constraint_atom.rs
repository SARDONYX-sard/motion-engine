//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetLocalRotationsConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpSetLocalRotationsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf81db8e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalRotationsConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"rotationA"`
    /// -   type: `hkRotation`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationA")]
    RotationA(Primitive<Rotation<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"rotationB"`
    /// -   type: `hkRotation`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationB")]
    RotationB(Primitive<Rotation<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalRotationsConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("rotationA" => RotationA(Primitive<Rotation<f32>>)),
    ("rotationB" => RotationB(Primitive<Rotation<f32>>)),
}

impl ByteDeSerialize for HkpSetLocalRotationsConstraintAtom {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
