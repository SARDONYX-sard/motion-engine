//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpReorientAction`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpReorientAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x2dc0ec6a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpReorientAction {
    /// # C++ Class Fields Info
    /// -   name:`"rotationAxis"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationAxis")]
    RotationAxis(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"upAxis"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upAxis")]
    UpAxis(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpReorientAction, "@name",
    ("rotationAxis" => RotationAxis(Vector4<f32>)),
    ("upAxis" => UpAxis(Vector4<f32>)),
    ("strength" => Strength(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
}
