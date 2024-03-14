//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexFloatDataChannel`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxVertexFloatDataChannel`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xbeeb397c`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexFloatDataChannel {
    /// # C++ Class Fields Info
    /// -   name:`"perVertexFloats"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "perVertexFloats")]
    PerVertexFloats(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"dimensions"`
    /// -   type: `enum VertexFloatDimensions`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dimensions")]
    Dimensions(Primitive<VertexFloatDimensions>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexFloatDataChannel, "@name",
    ("perVertexFloats" => PerVertexFloats(HkArrayRef<Primitive<f32>>)),
    ("dimensions" => Dimensions(Primitive<VertexFloatDimensions>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VertexFloatDimensions {
    #[serde(rename = "FLOAT")]
    Float = 0,
    #[serde(rename = "DISTANCE")]
    Distance = 1,
    #[serde(rename = "ANGLE")]
    Angle = 2,
}
