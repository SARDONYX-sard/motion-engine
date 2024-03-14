//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbExpressionCondition`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExpressionCondition<'a> {
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
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "compiledExpressionSet", skip_serializing)]
    CompiledExpressionSet(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionCondition<'de>, "@name",
    ("expression" => Expression(Primitive<Cow<'de, str>>)),
    ("compiledExpressionSet" => CompiledExpressionSet(Cow<'de, str>)),
}
