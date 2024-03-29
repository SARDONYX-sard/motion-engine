//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkGizmoAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkGizmoAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x23aadfb6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkGizmoAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"label"`
    /// -   type: `char*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "label")]
    Label(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum GizmoType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<GizmoType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkGizmoAttribute<'de>, "@name",
    ("visible" => Visible(Primitive<bool>)),
    ("label" => Label(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<GizmoType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GizmoType {
    #[serde(rename = "POINT")]
    Point = 0,
    #[serde(rename = "SPHERE")]
    Sphere = 1,
    #[serde(rename = "PLANE")]
    Plane = 2,
    #[serde(rename = "ARROW")]
    Arrow = 3,
}
