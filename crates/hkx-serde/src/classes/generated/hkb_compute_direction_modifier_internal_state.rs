//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbComputeDirectionModifierInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbComputeDirectionModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x6ac054d7`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeDirectionModifierInternalState {
    /// # C++ Class Fields Info
    /// -   name:`"pointOut"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointOut")]
    PointOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"groundAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundAngleOut")]
    GroundAngleOut(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"upAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upAngleOut")]
    UpAngleOut(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"computedOutput"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computedOutput")]
    ComputedOutput(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeDirectionModifierInternalState, "@name",
    ("pointOut" => PointOut(Vector4<f32>)),
    ("groundAngleOut" => GroundAngleOut(Primitive<f32>)),
    ("upAngleOut" => UpAngleOut(Primitive<f32>)),
    ("computedOutput" => ComputedOutput(Primitive<bool>)),
}
