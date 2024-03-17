//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaMeshBinding`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaMeshBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x81d9950b`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaMeshBinding<'a> {
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
    /// -   name:`"originalSkeletonName"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalSkeletonName")]
    OriginalSkeletonName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeleton")]
    Skeleton(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"mappings"`
    /// -   type: `hkArray&lt;struct hkaMeshBindingMapping&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mappings")]
    Mappings(HkArrayClass<HkaMeshBindingMapping>),
    /// # C++ Class Fields Info
    /// -   name:`"boneFromSkinMeshTransforms"`
    /// -   type: `hkArray&lt;hkTransform&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneFromSkinMeshTransforms")]
    BoneFromSkinMeshTransforms(HkArrayMatrix4<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaMeshBinding<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("mesh" => Mesh(Primitive<Cow<'de, str>>)),
    ("originalSkeletonName" => OriginalSkeletonName(Primitive<Cow<'de, str>>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
    ("mappings" => Mappings(HkArrayClass<HkaMeshBindingMapping>)),
    ("boneFromSkinMeshTransforms" => BoneFromSkinMeshTransforms(HkArrayMatrix4<Transform<f32>>)),
}
