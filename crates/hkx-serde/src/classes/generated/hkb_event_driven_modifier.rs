//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbEventDrivenModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventDrivenModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkbModifierWrapper`/`0x3697e044`
/// - signature: `0x7ed3f44e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventDrivenModifier {
    /// # C++ Class Fields Info
    /// -   name:`"activateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activateEventId")]
    ActivateEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivateEventId")]
    DeactivateEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"activeByDefault"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeByDefault")]
    ActiveByDefault(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventDrivenModifier, "@name",
    ("activateEventId" => ActivateEventId(Primitive<i32>)),
    ("deactivateEventId" => DeactivateEventId(Primitive<i32>)),
    ("activeByDefault" => ActiveByDefault(Primitive<bool>)),
    ("isActive" => IsActive(Primitive<bool>)),
}
