//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbMirroredSkeletonInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbMirroredSkeletonInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc6c2da4f`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbMirroredSkeletonInfo {
    /// # C++ Class Fields Info
    /// -   name:`"mirrorAxis"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirrorAxis")]
    MirrorAxis(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bonePairMap"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bonePairMap")]
    BonePairMap(HkArrayRef<Primitive<i16>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbMirroredSkeletonInfo, "@name",
    ("mirrorAxis" => MirrorAxis(Vector4<f32>)),
    ("bonePairMap" => BonePairMap(HkArrayRef<Primitive<i16>>)),
}
