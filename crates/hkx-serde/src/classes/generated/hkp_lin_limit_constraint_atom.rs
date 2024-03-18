//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinLimitConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpLinLimitConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xa44d1b07`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinLimitConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"axisIndex"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axisIndex")]
    AxisIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "min")]
    Min(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"max"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "max")]
    Max(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinLimitConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("axisIndex" => AxisIndex(Primitive<u8>)),
    ("min" => Min(Primitive<f32>)),
    ("max" => Max(Primitive<f32>)),
}
