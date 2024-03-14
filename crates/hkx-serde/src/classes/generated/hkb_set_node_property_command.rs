//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbSetNodePropertyCommand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSetNodePropertyCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc5160b64`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetNodePropertyCommand<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"propertyName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "propertyName")]
    PropertyName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"propertyValue"`
    /// -   type: `struct hkbVariableValue`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "propertyValue")]
    PropertyValue(HkbVariableValue),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetNodePropertyCommand<'de>, "@name",
    ("characterId" => CharacterId(Primitive<u64>)),
    ("nodeName" => NodeName(Primitive<Cow<'de, str>>)),
    ("propertyName" => PropertyName(Primitive<Cow<'de, str>>)),
    ("propertyValue" => PropertyValue(HkbVariableValue)),
    ("padding" => Padding(Primitive<i32>)),
}
