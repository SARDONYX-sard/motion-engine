//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSiStateTaggingGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSiStateTaggingGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xf0826fc1`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BSiStateTaggingGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"iStateToSetAs"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iStateToSetAs")]
    IStateToSetAs(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"iPriority"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iPriority")]
    IPriority(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BSiStateTaggingGenerator<'de>, "@name",
    ("pDefaultGenerator" => PDefaultGenerator(Primitive<Cow<'de, str>>)),
    ("iStateToSetAs" => IStateToSetAs(Primitive<i32>)),
    ("iPriority" => IPriority(Primitive<i32>)),
}
