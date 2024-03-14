//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRegisteredGenerator`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbRegisteredGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0x58b1d082`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRegisteredGenerator<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generator")]
    Generator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"relativePosition"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativePosition")]
    RelativePosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"relativeDirection"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeDirection")]
    RelativeDirection(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRegisteredGenerator<'de>, "@name",
    ("generator" => Generator(Cow<'de, str>)),
    ("relativePosition" => RelativePosition(Vector4<f32>)),
    ("relativeDirection" => RelativeDirection(Vector4<f32>)),
}
