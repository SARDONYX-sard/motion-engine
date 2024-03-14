//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkpSphereRepShape`/`0xe7eca7eb`
/// - signature: `0xf8f74f85`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexShape {
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexShape, "@name",
    ("radius" => Radius(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeldResult {
    #[serde(rename = "WELD_RESULT_REJECT_CONTACT_POINT")]
    WeldResultRejectContactPoint = 0,
    #[serde(rename = "WELD_RESULT_ACCEPT_CONTACT_POINT_MODIFIED")]
    WeldResultAcceptContactPointModified = 1,
    #[serde(rename = "WELD_RESULT_ACCEPT_CONTACT_POINT_UNMODIFIED")]
    WeldResultAcceptContactPointUnmodified = 2,
}
