//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMaterial`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x33be6570`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMaterial {
    /// # C++ Class Fields Info
    /// -   name:`"responseType"`
    /// -   type: `enum ResponseType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "responseType")]
    ResponseType(Primitive<ResponseType>),
    /// # C++ Class Fields Info
    /// -   name:`"rollingFrictionMultiplier"`
    /// -   type: `hkHalf`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rollingFrictionMultiplier")]
    RollingFrictionMultiplier(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"restitution"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restitution")]
    Restitution(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMaterial, "@name",
    ("responseType" => ResponseType(Primitive<ResponseType>)),
    ("rollingFrictionMultiplier" => RollingFrictionMultiplier(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("restitution" => Restitution(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "RESPONSE_INVALID")]
    ResponseInvalid = 0,
    #[serde(rename = "RESPONSE_SIMPLE_CONTACT")]
    ResponseSimpleContact = 1,
    #[serde(rename = "RESPONSE_REPORTING")]
    ResponseReporting = 2,
    #[serde(rename = "RESPONSE_NONE")]
    ResponseNone = 3,
    #[serde(rename = "RESPONSE_MAX_ID")]
    ResponseMaxId = 4,
}
