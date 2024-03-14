//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbVariableInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbVariableInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 6
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x9e746ba2`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableInfo {
    /// # C++ Class Fields Info
    /// -   name:`"role"`
    /// -   type: `struct hkbRoleAttribute`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "role")]
    Role(HkbRoleAttribute),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum VariableType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(VariableType),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableInfo, "@name",
    ("role" => Role(HkbRoleAttribute)),
    ("type" => Type(VariableType)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VariableType {
    #[serde(rename = "VARIABLE_TYPE_INVALID")]
    VariableTypeInvalid = -1,
    #[serde(rename = "VARIABLE_TYPE_BOOL")]
    VariableTypeBool = 0,
    #[serde(rename = "VARIABLE_TYPE_INT8")]
    VariableTypeInt8 = 1,
    #[serde(rename = "VARIABLE_TYPE_INT16")]
    VariableTypeInt16 = 2,
    #[serde(rename = "VARIABLE_TYPE_INT32")]
    VariableTypeInt32 = 3,
    #[serde(rename = "VARIABLE_TYPE_REAL")]
    VariableTypeReal = 4,
    #[serde(rename = "VARIABLE_TYPE_POINTER")]
    VariableTypePointer = 5,
    #[serde(rename = "VARIABLE_TYPE_VECTOR3")]
    VariableTypeVector3 = 6,
    #[serde(rename = "VARIABLE_TYPE_VECTOR4")]
    VariableTypeVector4 = 7,
    #[serde(rename = "VARIABLE_TYPE_QUATERNION")]
    VariableTypeQuaternion = 8,
}
