//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMultiSphereShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMultiSphereShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkpSphereRepShape`/`0xe7eca7eb`
/// - signature: `0x61a590fc`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMultiSphereShape {
    /// # C++ Class Fields Info
    /// -   name:`"numSpheres"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSpheres")]
    NumSpheres(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"spheres"`
    /// -   type: `hkVector4[8]`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spheres")]
    Spheres([Vector4<f32>; 8]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMultiSphereShape, "@name",
    ("numSpheres" => NumSpheres(Primitive<i32>)),
    ("spheres" => Spheres([Vector4<f32>; 8])),
}
