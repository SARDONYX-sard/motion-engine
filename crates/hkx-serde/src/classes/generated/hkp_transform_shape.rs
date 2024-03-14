//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpTransformShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTransformShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0x787ef513`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTransformShape {
    /// # C++ Class Fields Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape")]
    ChildShape(HkpSingleShapeContainer),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeSize", skip_serializing)]
    ChildShapeSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Transform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTransformShape, "@name",
    ("childShape" => ChildShape(HkpSingleShapeContainer)),
    ("childShapeSize" => ChildShapeSize(Primitive<i32>)),
    ("rotation" => Rotation(Quaternion<f32>)),
    ("transform" => Transform(Transform<f32>)),
}
