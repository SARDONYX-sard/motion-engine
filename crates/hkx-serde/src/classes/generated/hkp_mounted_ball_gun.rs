//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpMountedBallGun`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMountedBallGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpBallGun`/`0x57b06d35`
/// - signature: `0x6791ffce`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMountedBallGun {
    /// # C++ Class Fields Info
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMountedBallGun, "@name",
    ("position" => Position(Vector4<f32>)),
}
