//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkIndexedTransformSet`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkIndexedTransformSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x87fe6b5c`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkIndexedTransformSet<'a> {
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
    /// -   name:`"matrices"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matrices")]
    Matrices(HkArrayVector<Matrix4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"inverseMatrices"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inverseMatrices")]
    InverseMatrices(HkArrayVector<Matrix4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"matricesOrder"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matricesOrder")]
    MatricesOrder(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"matricesNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matricesNames")]
    MatricesNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"indexMappings"`
    /// -   type: `hkArray&lt;struct hkMeshBoneIndexMapping&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexMappings")]
    IndexMappings(HkArrayClass<HkMeshBoneIndexMapping>),
    /// # C++ Class Fields Info
    /// -   name:`"allMatricesAreAffine"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allMatricesAreAffine")]
    AllMatricesAreAffine(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkIndexedTransformSet<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("matrices" => Matrices(HkArrayVector<Matrix4<f32>>)),
    ("inverseMatrices" => InverseMatrices(HkArrayVector<Matrix4<f32>>)),
    ("matricesOrder" => MatricesOrder(HkArrayRef<Primitive<i16>>)),
    ("matricesNames" => MatricesNames(HkArrayStringPtr<'de>)),
    ("indexMappings" => IndexMappings(HkArrayClass<HkMeshBoneIndexMapping>)),
    ("allMatricesAreAffine" => AllMatricesAreAffine(Primitive<bool>)),
}
