//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbManualSelectorGeneratorInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbManualSelectorGeneratorInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x492c6137`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbManualSelectorGeneratorInternalState {
    /// # C++ Class Fields Info
    /// -   name:`"currentGeneratorIndex"`
    /// -   type: `hkInt8`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentGeneratorIndex")]
    CurrentGeneratorIndex(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbManualSelectorGeneratorInternalState, "@name",
    ("currentGeneratorIndex" => CurrentGeneratorIndex(Primitive<i8>)),
}
