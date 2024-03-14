//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpDashpotAction`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpDashpotAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpBinaryAction`/`0xc00f3403`
/// - signature: `0x50746c6e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDashpotAction {
    /// # C++ Class Fields Info
    /// -   name:`"point"`
    /// -   type: `hkVector4[2]`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "point")]
    Point([Vector4<f32>; 2]),
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
    /// # C++ Class Fields Info
    /// -   name:`"impulse"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "impulse")]
    Impulse(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDashpotAction, "@name",
    ("point" => Point([Vector4<f32>; 2])),
    ("strength" => Strength(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("impulse" => Impulse(Vector4<f32>)),
}
