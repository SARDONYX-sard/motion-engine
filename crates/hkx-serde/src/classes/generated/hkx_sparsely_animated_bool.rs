//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSparselyAnimatedBool`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxSparselyAnimatedBool`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x7a894596`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSparselyAnimatedBool {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"bools"`
    /// -   type: `hkArray<hkBool>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bools")]
    Bools(HkArrayRef<Primitive<bool>>),
    /// # C++ Class Fields Info
    /// -   name:`"times"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "times")]
    Times(HkArrayRef<Primitive<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedBool, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bools" => Bools(HkArrayRef<Primitive<bool>>)),
    ("times" => Times(HkArrayRef<Primitive<f32>>)),
}
