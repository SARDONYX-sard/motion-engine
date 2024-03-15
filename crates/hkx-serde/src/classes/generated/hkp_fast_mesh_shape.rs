//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpFastMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpFastMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpMeshShape`/`0x3bf12c0f`
/// - signature: `0x3d3da311`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpFastMeshShape {
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"scaling"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaling", default)]
    Scaling(Vector4<f32>),
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBitsForSubpartIndex", default)]
    NumBitsForSubpartIndex(Primitive<i32>),
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"subparts"`
    /// -   type: `hkArray&lt;struct hkpMeshShapeSubpart&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subparts", default)]
    Subparts(HkArrayClass<HkpMeshShapeSubpart>),
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo", default)]
    WeldingInfo(HkArrayRef<Primitive<u16>>),
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType", default)]
    WeldingType(Primitive<WeldingType>),
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius", default)]
    Radius(Primitive<f32>),
    /// # C++ Parent class(`hkpMeshShape`, parent: `hkpShapeCollection`) field Info
    /// -   name:`"pad"`
    /// -   type: `hkInt32[3]`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad", default)]
    Pad([Primitive<i32>; 3]),

    /// # C++ Parent class(`hkpShapeCollection`, parent: `hkpShape`) field Info
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disableWelding", default)]
    DisableWelding(Primitive<bool>),
    /// # C++ Parent class(`hkpShapeCollection`, parent: `hkpShape`) field Info
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collectionType", default)]
    CollectionType(Primitive<CollectionType>),

    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpShape`, parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", default, skip_serializing)]
    Type(Primitive<Unknown>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpFastMeshShape, "@name",
    ("scaling" => Scaling(Vector4<f32>)),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(Primitive<i32>)),
    ("subparts" => Subparts(HkArrayClass<HkpMeshShapeSubpart>)),
    ("weldingInfo" => WeldingInfo(HkArrayRef<Primitive<u16>>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("radius" => Radius(Primitive<f32>)),
    ("pad" => Pad([Primitive<i32>; 3])),
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<Unknown>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}
