//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbMoveCharacterModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbMoveCharacterModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x8f7492a0`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbMoveCharacterModifier {
    /// # C++ Class Fields Info
    /// -   name:`"offsetPerSecondMS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetPerSecondMS")]
    OffsetPerSecondMs(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbMoveCharacterModifier, "@name",
    ("offsetPerSecondMS" => OffsetPerSecondMs(Vector4<f32>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
}
