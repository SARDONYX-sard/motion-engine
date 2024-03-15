//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCompiledExpressionSet`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCompiledExpressionSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3a7d76cc`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCompiledExpressionSet {
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"rpn"`
    /// -   type: `hkArray&lt;struct hkbCompiledExpressionSetToken&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rpn", default)]
    Rpn(HkArrayClass<HkbCompiledExpressionSetToken>),
    /// # C++ Class Fields Info
    /// -   name:`"expressionToRpnIndex"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expressionToRpnIndex", default)]
    ExpressionToRpnIndex(HkArrayRef<Primitive<i32>>),
    /// # C++ Class Fields Info
    /// -   name:`"numExpressions"`
    /// -   type: `hkInt8`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numExpressions", default)]
    NumExpressions(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCompiledExpressionSet, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rpn" => Rpn(HkArrayClass<HkbCompiledExpressionSetToken>)),
    ("expressionToRpnIndex" => ExpressionToRpnIndex(HkArrayRef<Primitive<i32>>)),
    ("numExpressions" => NumExpressions(Primitive<i8>)),
}
