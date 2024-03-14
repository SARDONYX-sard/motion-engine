//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStringCondition`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStringCondition`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkbCondition`/`0xda8c7d7d`
/// - signature: `0x5ab50487`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStringCondition<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"conditionString"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "conditionString")]
    ConditionString(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStringCondition<'de>, "@name",
    ("conditionString" => ConditionString(Primitive<Cow<'de, str>>)),
}
