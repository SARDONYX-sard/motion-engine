//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinkedCollidable`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpLinkedCollidable`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 92
/// -    vtable: false
/// -    parent: `hkpCollidable`/`0x9a0e42a5`
/// - signature: `0xe1a81497`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinkedCollidable {
    /// # C++ Parent class(`hkpCollidable`, parent: `hkpCdBody`) field Info
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ownerOffset", default, skip_serializing)]
    OwnerOffset(Primitive<i8>),
    /// # C++ Parent class(`hkpCollidable`, parent: `hkpCdBody`) field Info
    /// -   name:`"forceCollideOntoPpu"`
    /// -   type: `hkUint8`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceCollideOntoPpu", default)]
    ForceCollideOntoPpu(Primitive<u8>),
    /// # C++ Parent class(`hkpCollidable`, parent: `hkpCdBody`) field Info
    /// -   name:`"shapeSizeOnSpu"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "shapeSizeOnSpu", default, skip_serializing)]
    ShapeSizeOnSpu(Primitive<u16>),
    /// # C++ Parent class(`hkpCollidable`, parent: `hkpCdBody`) field Info
    /// -   name:`"broadPhaseHandle"`
    /// -   type: `struct hkpTypedBroadPhaseHandle`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseHandle", default)]
    BroadPhaseHandle(HkpTypedBroadPhaseHandle),
    /// # C++ Parent class(`hkpCollidable`, parent: `hkpCdBody`) field Info
    /// -   name:`"boundingVolumeData"`
    /// -   type: `struct hkpCollidableBoundingVolumeData`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boundingVolumeData", default, skip_serializing)]
    BoundingVolumeData(HkpCollidableBoundingVolumeData),
    /// # C++ Parent class(`hkpCollidable`, parent: `hkpCdBody`) field Info
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowedPenetrationDepth", default)]
    AllowedPenetrationDepth(Primitive<f32>),

    /// # C++ Parent class(`hkpCdBody`, parent: `None`) field Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape", default)]
    Shape(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpCdBody`, parent: `None`) field Info
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapeKey", default)]
    ShapeKey(Primitive<u32>),
    /// # C++ Parent class(`hkpCdBody`, parent: `None`) field Info
    /// -   name:`"motion"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "motion", default, skip_serializing)]
    Motion(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpCdBody`, parent: `None`) field Info
    /// -   name:`"parent"`
    /// -   type: `struct hkpCdBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parent", default, skip_serializing)]
    Parent(Primitive<Cow<'a, str>>),

    /// # C++ Class Fields Info
    /// -   name:`"collisionEntries"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionEntries", default, skip_serializing)]
    CollisionEntries(HkArrayRef<Primitive<()>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinkedCollidable, "@name",
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("forceCollideOntoPpu" => ForceCollideOntoPpu(Primitive<u8>)),
    ("shapeSizeOnSpu" => ShapeSizeOnSpu(Primitive<u16>)),
    ("broadPhaseHandle" => BroadPhaseHandle(HkpTypedBroadPhaseHandle)),
    ("boundingVolumeData" => BoundingVolumeData(HkpCollidableBoundingVolumeData)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(Primitive<f32>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("shapeKey" => ShapeKey(Primitive<u32>)),
    ("motion" => Motion(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
    ("collisionEntries" => CollisionEntries(HkArrayRef<Primitive<()>>)),
}
