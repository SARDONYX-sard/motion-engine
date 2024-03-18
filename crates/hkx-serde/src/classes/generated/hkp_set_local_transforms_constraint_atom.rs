//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetLocalTransformsConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpSetLocalTransformsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x6e2a5198`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalTransformsConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"transformA"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformA")]
    TransformA(Transform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"transformB"`
    /// -   type: `hkTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformB")]
    TransformB(Transform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalTransformsConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("transformA" => TransformA(Transform<f32>)),
    ("transformB" => TransformB(Transform<f32>)),
}
