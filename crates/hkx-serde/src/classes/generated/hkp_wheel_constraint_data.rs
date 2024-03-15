//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWheelConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpWheelConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 352
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0xb4c46671`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWheelConstraintData {
    /// # C++ Parent class(`hkpConstraintData`, parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),

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

    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpWheelConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms", default)]
    Atoms(HkpWheelConstraintDataAtoms),
    /// # C++ Class Fields Info
    /// -   name:`"initialAxleInB"`
    /// -   type: `hkVector4`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialAxleInB", default)]
    InitialAxleInB(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"initialSteeringAxisInB"`
    /// -   type: `hkVector4`
    /// - offset: 336
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialSteeringAxisInB", default)]
    InitialSteeringAxisInB(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWheelConstraintData, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("atoms" => Atoms(HkpWheelConstraintDataAtoms)),
    ("initialAxleInB" => InitialAxleInB(Vector4<f32>)),
    ("initialSteeringAxisInB" => InitialSteeringAxisInB(Vector4<f32>)),
}
