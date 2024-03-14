//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbVariableBindingSet`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbVariableBindingSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x338ad4ff`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableBindingSet {
    /// # C++ Class Fields Info
    /// -   name:`"bindings"`
    /// -   type: `hkArray&lt;struct hkbVariableBindingSetBinding&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindings")]
    Bindings(HkArrayClass<HkbVariableBindingSetBinding>),
    /// # C++ Class Fields Info
    /// -   name:`"indexOfBindingToEnable"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfBindingToEnable")]
    IndexOfBindingToEnable(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"hasOutputBinding"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "hasOutputBinding", skip_serializing)]
    HasOutputBinding(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableBindingSet, "@name",
    ("bindings" => Bindings(HkArrayClass<HkbVariableBindingSetBinding>)),
    ("indexOfBindingToEnable" => IndexOfBindingToEnable(Primitive<i32>)),
    ("hasOutputBinding" => HasOutputBinding(Primitive<bool>)),
}
