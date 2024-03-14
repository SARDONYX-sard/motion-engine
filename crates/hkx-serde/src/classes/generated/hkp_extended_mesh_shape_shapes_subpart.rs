//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpExtendedMeshShapeShapesSubpart`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpExtendedMeshShapeShapesSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpExtendedMeshShapeSubpart`/`0xf4608207`
/// - signature: `0xf204b155`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeShapesSubpart<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"childShapes"`
    /// -   type: `hkArray&lt;hkpConvexShape*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShapes")]
    ChildShapes(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translation")]
    Translation(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeShapesSubpart<'de>, "@name",
    ("childShapes" => ChildShapes(HkArrayRef<Cow<'de, str>>)),
    ("rotation" => Rotation(Quaternion<f32>)),
    ("translation" => Translation(Vector4<f32>)),
}
