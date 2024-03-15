//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraphInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBehaviorGraphInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x8699b6eb`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphInternalState<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"nodeInternalStateInfos"`
    /// -   type: `hkArray&lt;hkbNodeInternalStateInfo*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeInternalStateInfos")]
    NodeInternalStateInfos(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"variableValueSet"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableValueSet")]
    VariableValueSet(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphInternalState<'de>, "@name",
    ("nodeInternalStateInfos" => NodeInternalStateInfos(HkArrayRef<Cow<'de, str>>)),
    ("variableValueSet" => VariableValueSet(Primitive<Cow<'de, str>>)),
}
