//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRoleAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbRoleAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0x3eb2e082`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRoleAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"role"`
    /// -   type: `enum Role`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "role")]
    Role(Primitive<Role>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags RoleFlags`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<RoleFlags>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRoleAttribute, "@name",
    ("role" => Role(Primitive<Role>)),
    ("flags" => Flags(Primitive<RoleFlags>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "ROLE_DEFAULT")]
    RoleDefault = 0,
    #[serde(rename = "ROLE_FILE_NAME")]
    RoleFileName = 1,
    #[serde(rename = "ROLE_BONE_INDEX")]
    RoleBoneIndex = 2,
    #[serde(rename = "ROLE_BONE_INDEX_MAP")]
    RoleBoneIndexMap = 3,
    #[serde(rename = "ROLE_EVENT_ID")]
    RoleEventId = 4,
    #[serde(rename = "ROLE_VARIABLE_INDEX")]
    RoleVariableIndex = 5,
    #[serde(rename = "ROLE_ATTRIBUTE_INDEX")]
    RoleAttributeIndex = 6,
    #[serde(rename = "ROLE_TIME")]
    RoleTime = 7,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoleFlags {
    #[serde(rename = "FLAG_NONE")]
    FlagNone = 0,
    #[serde(rename = "FLAG_RAGDOLL")]
    FlagRagdoll = 1,
    #[serde(rename = "FLAG_NORMALIZED")]
    FlagNormalized = 2,
    #[serde(rename = "FLAG_NOT_VARIABLE")]
    FlagNotVariable = 4,
    #[serde(rename = "FLAG_HIDDEN")]
    FlagHidden = 8,
    #[serde(rename = "FLAG_OUTPUT")]
    FlagOutput = 16,
    #[serde(rename = "FLAG_NOT_CHARACTER_PROPERTY")]
    FlagNotCharacterProperty = 32,
}
