//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDampingModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbDampingModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x9a040f03`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDampingModifier {
    /// # C++ Class Fields Info
    /// -   name:`"kP"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kP")]
    KP(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"kI"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kI")]
    KI(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"kD"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kD")]
    KD(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enableScalarDamping"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableScalarDamping")]
    EnableScalarDamping(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"enableVectorDamping"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableVectorDamping")]
    EnableVectorDamping(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"rawValue"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawValue")]
    RawValue(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rawVector"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawVector")]
    RawVector(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedVector")]
    DampedVector(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifier, "@name",
    ("kP" => KP(Primitive<f32>)),
    ("kI" => KI(Primitive<f32>)),
    ("kD" => KD(Primitive<f32>)),
    ("enableScalarDamping" => EnableScalarDamping(Primitive<bool>)),
    ("enableVectorDamping" => EnableVectorDamping(Primitive<bool>)),
    ("rawValue" => RawValue(Primitive<f32>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("rawVector" => RawVector(Vector4<f32>)),
    ("dampedVector" => DampedVector(Vector4<f32>)),
    ("vecErrorSum" => VecErrorSum(Vector4<f32>)),
    ("vecPreviousError" => VecPreviousError(Vector4<f32>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}
