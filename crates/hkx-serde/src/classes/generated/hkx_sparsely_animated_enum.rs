//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSparselyAnimatedEnum`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkxSparselyAnimatedEnum`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkxSparselyAnimatedInt`/`0xca961951`
/// - signature: `0x68a47b64`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSparselyAnimatedEnum<'a> {
    /// # C++ Parent class(`hkxSparselyAnimatedInt` => parent: `hkReferencedObject`) field Info
    /// -   name:`"ints"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ints")]
    Ints(HkArrayNum<i32>),
    /// # C++ Parent class(`hkxSparselyAnimatedInt` => parent: `hkReferencedObject`) field Info
    /// -   name:`"times"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "times")]
    Times(HkArrayNum<f32>),

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
    //
    /// # C++ Class Fields Info
    /// -   name:`"enum"`
    /// -   type: `struct hkxEnum*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enum")]
    Enum(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedEnum<'de>, "@name",
    ("ints" => Ints(HkArrayNum<i32>)),
    ("times" => Times(HkArrayNum<f32>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("enum" => Enum(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkxSparselyAnimatedEnum<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
