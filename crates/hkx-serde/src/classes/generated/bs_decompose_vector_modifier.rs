//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `BSDecomposeVectorModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSDecomposeVectorModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x31f6b8b6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsDecomposeVectorModifier {
    /// # C++ Class Fields Info
    /// -   name:`"vector"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vector")]
    Vector(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"x"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "x")]
    X(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"y"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "y")]
    Y(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"z"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "z")]
    Z(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"w"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "w")]
    W(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsDecomposeVectorModifier, "@name",
    ("vector" => Vector(Vector4<f32>)),
    ("x" => X(Primitive<f32>)),
    ("y" => Y(Primitive<f32>)),
    ("z" => Z(Primitive<f32>)),
    ("w" => W(Primitive<f32>)),
}
