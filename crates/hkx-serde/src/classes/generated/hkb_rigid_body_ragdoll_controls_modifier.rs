//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRigidBodyRagdollControlsModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbRigidBodyRagdollControlsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xaa87d1eb`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRigidBodyRagdollControlsModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbRigidBodyRagdollControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbRigidBodyRagdollControlData),
    /// # C++ Class Fields Info
    /// -   name:`"bones"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRigidBodyRagdollControlsModifier<'de>, "@name",
    ("controlData" => ControlData(HkbRigidBodyRagdollControlData)),
    ("bones" => Bones(Primitive<Cow<'de, str>>)),
}
