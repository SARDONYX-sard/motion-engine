//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbExpressionCondition`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbExpressionCondition`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: true
/// -    parent: `hkbCondition`/`0xda8c7d7d`
/// - signature: `0x1c3c1045`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExpressionCondition<'a> {
    // C++ Parent class(`hkbCondition` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"expression"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expression")]
    Expression(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"compiledExpressionSet"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "compiledExpressionSet", skip_serializing)]
    CompiledExpressionSet(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionCondition<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("expression" => Expression(Primitive<Cow<'de, str>>)),
    ("compiledExpressionSet" => CompiledExpressionSet(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkbExpressionCondition<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
