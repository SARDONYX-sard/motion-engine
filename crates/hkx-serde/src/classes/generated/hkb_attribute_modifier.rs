//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbAttributeModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbAttributeModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x1245d97d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAttributeModifier {
    /// # C++ Class Fields Info
    /// -   name:`"assignments"`
    /// -   type: `hkArray&lt;struct hkbAttributeModifierAssignment&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "assignments")]
    Assignments(HkArrayClass<HkbAttributeModifierAssignment>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttributeModifier, "@name",
    ("assignments" => Assignments(HkArrayClass<HkbAttributeModifierAssignment>)),
}
