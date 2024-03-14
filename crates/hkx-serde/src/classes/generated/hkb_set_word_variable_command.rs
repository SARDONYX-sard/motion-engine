//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSetWordVariableCommand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSetWordVariableCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xf3ae5fca`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetWordVariableCommand {
    /// # C++ Class Fields Info
    /// -   name:`"quadValue"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quadValue")]
    QuadValue(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"variableId"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableId")]
    VariableId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `struct hkbVariableValue`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(HkbVariableValue),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum VariableType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<VariableType>),
    /// # C++ Class Fields Info
    /// -   name:`"global"`
    /// -   type: `hkBool`
    /// - offset: 49
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "global")]
    Global(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetWordVariableCommand, "@name",
    ("quadValue" => QuadValue(Vector4<f32>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("variableId" => VariableId(Primitive<i32>)),
    ("value" => Value(HkbVariableValue)),
    ("type" => Type(Primitive<VariableType>)),
    ("global" => Global(Primitive<bool>)),
}
