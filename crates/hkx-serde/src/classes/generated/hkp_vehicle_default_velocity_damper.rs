//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultVelocityDamper`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpVehicleDefaultVelocityDamper`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkpVehicleVelocityDamper`/`0xda8c7d7d`
/// - signature: `0x741b8d9e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultVelocityDamper {
    // `hkpVehicleVelocityDamper`(Parent class) has no fields

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"normalSpinDamping"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalSpinDamping")]
    NormalSpinDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionSpinDamping"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionSpinDamping")]
    CollisionSpinDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionThreshold"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionThreshold")]
    CollisionThreshold(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultVelocityDamper, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("normalSpinDamping" => NormalSpinDamping(Primitive<f32>)),
    ("collisionSpinDamping" => CollisionSpinDamping(Primitive<f32>)),
    ("collisionThreshold" => CollisionThreshold(Primitive<f32>)),
}
