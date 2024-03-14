//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkGizmoAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkGizmoAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x23aadfb6`
/// -   version: 0
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
    Type(GizmoType),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkGizmoAttribute<'de>, "@name",
    ("visible" => Visible(Primitive<bool>)),
    ("label" => Label(Primitive<Cow<'de, str>>)),
    ("type" => Type(GizmoType)),
}

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
