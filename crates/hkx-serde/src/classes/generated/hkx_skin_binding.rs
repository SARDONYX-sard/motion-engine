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
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

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
    BindPose(HkArrayMatrix4<Matrix4<f32>>),
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
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("mesh" => Mesh(Primitive<Cow<'de, str>>)),
    ("nodeNames" => NodeNames(HkArrayStringPtr<'de>)),
    ("bindPose" => BindPose(HkArrayMatrix4<Matrix4<f32>>)),
    ("initSkinTransform" => InitSkinTransform(Matrix4<f32>)),
}
