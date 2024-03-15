//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpModifierConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xb13fef1f`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpModifierConstraintAtom<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"modifierAtomSize"`
    /// -   type: `hkUint16`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "modifierAtomSize")]
    ModifierAtomSize(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childSize")]
    ChildSize(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"child"`
    /// -   type: `struct hkpConstraintAtom*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "child")]
    Child(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"pad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad")]
    Pad([Primitive<u32>; 2]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpModifierConstraintAtom<'de>, "@name",
    ("modifierAtomSize" => ModifierAtomSize(Primitive<u16>)),
    ("childSize" => ChildSize(Primitive<u16>)),
    ("child" => Child(Primitive<Cow<'de, str>>)),
    ("pad" => Pad([Primitive<u32>; 2])),
}
