//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpEntityExtendedListeners`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpEntityExtendedListeners`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xf557023c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpEntityExtendedListeners<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"activationListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activationListeners", skip_serializing)]
    ActivationListeners(HkpEntitySmallArraySerializeOverrideType<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"entityListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "entityListeners", skip_serializing)]
    EntityListeners(HkpEntitySmallArraySerializeOverrideType<'a>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntityExtendedListeners<'de>, "@name",
    ("activationListeners" => ActivationListeners(HkpEntitySmallArraySerializeOverrideType<'de>)),
    ("entityListeners" => EntityListeners(HkpEntitySmallArraySerializeOverrideType<'de>)),
}
