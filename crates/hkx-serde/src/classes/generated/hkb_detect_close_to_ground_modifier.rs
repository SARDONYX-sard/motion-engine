//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDetectCloseToGroundModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbDetectCloseToGroundModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x981687b2`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDetectCloseToGroundModifier {
    /// # C++ Class Fields Info
    /// -   name:`"closeToGroundEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "closeToGroundEvent")]
    CloseToGroundEvent(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"closeToGroundHeight"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "closeToGroundHeight")]
    CloseToGroundHeight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"animBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 66
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animBoneIndex")]
    AnimBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"isCloseToGround"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isCloseToGround", skip_serializing)]
    IsCloseToGround(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDetectCloseToGroundModifier, "@name",
    ("closeToGroundEvent" => CloseToGroundEvent(HkbEventProperty)),
    ("closeToGroundHeight" => CloseToGroundHeight(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("animBoneIndex" => AnimBoneIndex(Primitive<i16>)),
    ("isCloseToGround" => IsCloseToGround(Primitive<bool>)),
}
