//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConvexPieceMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexPieceMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0x38fd3d97`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexPieceMeshShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"convexPieceStream"`
    /// -   type: `struct hkpConvexPieceStreamData*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieceStream")]
    ConvexPieceStream(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"displayMesh"`
    /// -   type: `struct hkpShapeCollection*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "displayMesh")]
    DisplayMesh(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexPieceMeshShape<'de>, "@name",
    ("convexPieceStream" => ConvexPieceStream(Cow<'de, str>)),
    ("displayMesh" => DisplayMesh(Cow<'de, str>)),
    ("radius" => Radius(Primitive<f32>)),
}
