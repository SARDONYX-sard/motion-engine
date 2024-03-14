//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbTransformVectorModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbTransformVectorModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xf93e0e24`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTransformVectorModifier {
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translation")]
    Translation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vectorIn"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorIn")]
    VectorIn(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"vectorOut"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorOut")]
    VectorOut(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotateOnly"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotateOnly")]
    RotateOnly(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"inverse"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inverse")]
    Inverse(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"computeOnActivate"`
    /// -   type: `hkBool`
    /// - offset: 114
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computeOnActivate")]
    ComputeOnActivate(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"computeOnModify"`
    /// -   type: `hkBool`
    /// - offset: 115
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computeOnModify")]
    ComputeOnModify(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTransformVectorModifier, "@name",
    ("rotation" => Rotation(Quaternion<f32>)),
    ("translation" => Translation(Vector4<f32>)),
    ("vectorIn" => VectorIn(Vector4<f32>)),
    ("vectorOut" => VectorOut(Vector4<f32>)),
    ("rotateOnly" => RotateOnly(Primitive<bool>)),
    ("inverse" => Inverse(Primitive<bool>)),
    ("computeOnActivate" => ComputeOnActivate(Primitive<bool>)),
    ("computeOnModify" => ComputeOnModify(Primitive<bool>)),
}
