//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSpringAction`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSpringAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpBinaryAction`/`0xc00f3403`
/// - signature: `0x88fc09fa`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSpringAction {
    /// # C++ Class Fields Info
    /// -   name:`"lastForce"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastForce")]
    LastForce(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"positionAinA"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionAinA")]
    PositionAinA(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"positionBinB"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionBinB")]
    PositionBinB(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"restLength"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restLength")]
    RestLength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"onCompression"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onCompression")]
    OnCompression(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"onExtension"`
    /// -   type: `hkBool`
    /// - offset: 93
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onExtension")]
    OnExtension(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSpringAction, "@name",
    ("lastForce" => LastForce(Vector4<f32>)),
    ("positionAinA" => PositionAinA(Vector4<f32>)),
    ("positionBinB" => PositionBinB(Vector4<f32>)),
    ("restLength" => RestLength(Primitive<f32>)),
    ("strength" => Strength(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("onCompression" => OnCompression(Primitive<bool>)),
    ("onExtension" => OnExtension(Primitive<bool>)),
}
