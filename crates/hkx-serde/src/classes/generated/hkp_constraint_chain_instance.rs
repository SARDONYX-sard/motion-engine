//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintChainInstance`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintChainInstance`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkpConstraintInstance`/`0x34eba5f`
/// - signature: `0x7a490753`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintChainInstance<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"chainedEntities"`
    /// -   type: `hkArray&lt;hkpEntity*&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chainedEntities")]
    ChainedEntities(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"action"`
    /// -   type: `struct hkpConstraintChainInstanceAction*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "action")]
    Action(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintChainInstance<'de>, "@name",
    ("chainedEntities" => ChainedEntities(HkArrayRef<Cow<'de, str>>)),
    ("action" => Action(Primitive<Cow<'de, str>>)),
}
