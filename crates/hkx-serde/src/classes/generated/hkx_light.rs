//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxLight`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxLight`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x81c86d42`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxLight {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum LightType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(LightType),
    /// # C++ Class Fields Info
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"direction"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "direction")]
    Direction(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"color"`
    /// -   type: `hkUint32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "color")]
    Color(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"angle"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angle")]
    Angle(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxLight, "@name",
    ("type" => Type(LightType)),
    ("position" => Position(Vector4<f32>)),
    ("direction" => Direction(Vector4<f32>)),
    ("color" => Color(Primitive<u32>)),
    ("angle" => Angle(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LightType {
    #[serde(rename = "POINT_LIGHT")]
    PointLight = 0,
    #[serde(rename = "DIRECTIONAL_LIGHT")]
    DirectionalLight = 1,
    #[serde(rename = "SPOT_LIGHT")]
    SpotLight = 2,
}
