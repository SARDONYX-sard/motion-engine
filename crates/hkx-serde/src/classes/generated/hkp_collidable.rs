//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCollidable`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCollidable`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// -    parent: `hkpCdBody`/`0x54a4b841`
/// - signature: `0x9a0e42a5`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCollidable {
    /// # C++ Class Fields Info
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ownerOffset", skip_serializing)]
    OwnerOffset(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"forceCollideOntoPpu"`
    /// -   type: `hkUint8`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceCollideOntoPpu")]
    ForceCollideOntoPpu(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"shapeSizeOnSpu"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "shapeSizeOnSpu", skip_serializing)]
    ShapeSizeOnSpu(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseHandle"`
    /// -   type: `struct hkpTypedBroadPhaseHandle`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseHandle")]
    BroadPhaseHandle(HkpTypedBroadPhaseHandle),
    /// # C++ Class Fields Info
    /// -   name:`"boundingVolumeData"`
    /// -   type: `struct hkpCollidableBoundingVolumeData`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boundingVolumeData", skip_serializing)]
    BoundingVolumeData(HkpCollidableBoundingVolumeData),
    /// # C++ Class Fields Info
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidable, "@name",
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("forceCollideOntoPpu" => ForceCollideOntoPpu(Primitive<u8>)),
    ("shapeSizeOnSpu" => ShapeSizeOnSpu(Primitive<u16>)),
    ("broadPhaseHandle" => BroadPhaseHandle(HkpTypedBroadPhaseHandle)),
    ("boundingVolumeData" => BoundingVolumeData(HkpCollidableBoundingVolumeData)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForceCollideOntoPpuReasons {
    #[serde(rename = "FORCE_PPU_USER_REQUEST")]
    ForcePpuUserRequest = 1,
    #[serde(rename = "FORCE_PPU_SHAPE_REQUEST")]
    ForcePpuShapeRequest = 2,
    #[serde(rename = "FORCE_PPU_MODIFIER_REQUEST")]
    ForcePpuModifierRequest = 4,
    #[serde(rename = "FORCE_PPU_SHAPE_UNCHECKED")]
    ForcePpuShapeUnchecked = 8,
}
