//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbFootIkModifierInternalLegData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbFootIkModifierInternalLegData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xe5ca3677`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifierInternalLegData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"groundPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundPosition")]
    GroundPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footIkSolver"`
    /// -   type: `void*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "footIkSolver", skip_serializing)]
    FootIkSolver(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierInternalLegData<'de>, "@name",
    ("groundPosition" => GroundPosition(Vector4<f32>)),
    ("footIkSolver" => FootIkSolver(Cow<'de, str>)),
}
