//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCenterOfMassChangerModifierConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpCenterOfMassChangerModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0x1d7dbdd2`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCenterOfMassChangerModifierConstraintAtom<'a> {
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"modifierAtomSize"`
    /// -   type: `hkUint16`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "modifierAtomSize")]
    ModifierAtomSize(Primitive<u16>),
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"childSize"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childSize")]
    ChildSize(Primitive<u16>),
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"child"`
    /// -   type: `struct hkpConstraintAtom*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "child")]
    Child(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"pad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad")]
    Pad(CStyleArray<[u32; 2]>),

    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"displacementA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "displacementA")]
    DisplacementA(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"displacementB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "displacementB")]
    DisplacementB(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCenterOfMassChangerModifierConstraintAtom<'de>, "@name",
    ("modifierAtomSize" => ModifierAtomSize(Primitive<u16>)),
    ("childSize" => ChildSize(Primitive<u16>)),
    ("child" => Child(Primitive<Cow<'de, str>>)),
    ("pad" => Pad(CStyleArray<[u32; 2]>)),
    ("type" => Type(Primitive<AtomType>)),
    ("displacementA" => DisplacementA(Vector4<f32>)),
    ("displacementB" => DisplacementB(Vector4<f32>)),
}
