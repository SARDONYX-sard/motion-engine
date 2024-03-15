//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpShapeInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpShapeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xea7f1d08`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpShapeInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"isHierarchicalCompound"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isHierarchicalCompound")]
    IsHierarchicalCompound(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"hkdShapesCollected"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hkdShapesCollected")]
    HkdShapesCollected(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShapeNames")]
    ChildShapeNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"childTransforms"`
    /// -   type: `hkArray&lt;hkTransform&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childTransforms")]
    ChildTransforms(HkArrayVector<Transform<f32>>),
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
    HkpShapeInfo<'de>, "@name",
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("isHierarchicalCompound" => IsHierarchicalCompound(Primitive<bool>)),
    ("hkdShapesCollected" => HkdShapesCollected(Primitive<bool>)),
    ("childShapeNames" => ChildShapeNames(HkArrayStringPtr<'de>)),
    ("childTransforms" => ChildTransforms(HkArrayVector<Transform<f32>>)),
    ("transform" => Transform(Transform<f32>)),
}
