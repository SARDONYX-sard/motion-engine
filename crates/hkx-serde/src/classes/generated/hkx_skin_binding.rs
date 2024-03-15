//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSkinBinding`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxSkinBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5a93f338`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSkinBinding<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"mesh"`
    /// -   type: `struct hkxMesh*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mesh")]
    Mesh(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeNames")]
    NodeNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"bindPose"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindPose")]
    BindPose(HkArrayVector<Matrix4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"initSkinTransform"`
    /// -   type: `hkMatrix4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initSkinTransform")]
    InitSkinTransform(Matrix4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSkinBinding<'de>, "@name",
    ("mesh" => Mesh(Primitive<Cow<'de, str>>)),
    ("nodeNames" => NodeNames(HkArrayStringPtr<'de>)),
    ("bindPose" => BindPose(HkArrayVector<Matrix4<f32>>)),
    ("initSkinTransform" => InitSkinTransform(Matrix4<f32>)),
}
