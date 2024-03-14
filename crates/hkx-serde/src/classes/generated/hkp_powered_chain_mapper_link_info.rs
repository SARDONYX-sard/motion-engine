//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainMapperLinkInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPoweredChainMapperLinkInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xcf071a1b`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainMapperLinkInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"firstTargetIdx"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "firstTargetIdx")]
    FirstTargetIdx(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numTargets"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numTargets")]
    NumTargets(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"limitConstraint"`
    /// -   type: `struct hkpConstraintInstance*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitConstraint")]
    LimitConstraint(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperLinkInfo<'de>, "@name",
    ("firstTargetIdx" => FirstTargetIdx(Primitive<i32>)),
    ("numTargets" => NumTargets(Primitive<i32>)),
    ("limitConstraint" => LimitConstraint(Cow<'de, str>)),
}
