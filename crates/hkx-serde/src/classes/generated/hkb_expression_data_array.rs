//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbExpressionDataArray`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbExpressionDataArray`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x4b9ee1a2`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExpressionDataArray {
    /// # C++ Class Fields Info
    /// -   name:`"expressionsData"`
    /// -   type: `hkArray&lt;struct hkbExpressionData&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expressionsData")]
    ExpressionsData(HkArrayClass<HkbExpressionData>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionDataArray, "@name",
    ("expressionsData" => ExpressionsData(HkArrayClass<HkbExpressionData>)),
}
