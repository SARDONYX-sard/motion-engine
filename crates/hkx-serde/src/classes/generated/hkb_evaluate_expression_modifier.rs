//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbEvaluateExpressionModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEvaluateExpressionModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xf900f6be`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateExpressionModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"expressions"`
    /// -   type: `struct hkbExpressionDataArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expressions")]
    Expressions(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"compiledExpressionSet"`
    /// -   type: `void*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "compiledExpressionSet", skip_serializing)]
    CompiledExpressionSet(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"internalExpressionsData"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalExpressionsData", skip_serializing)]
    InternalExpressionsData(HkArrayRef<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateExpressionModifier<'de>, "@name",
    ("expressions" => Expressions(Cow<'de, str>)),
    ("compiledExpressionSet" => CompiledExpressionSet(Cow<'de, str>)),
    ("internalExpressionsData" => InternalExpressionsData(HkArrayRef<()>)),
}
