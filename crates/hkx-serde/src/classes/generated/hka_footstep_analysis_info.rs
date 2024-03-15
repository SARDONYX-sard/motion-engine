//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaFootstepAnalysisInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaFootstepAnalysisInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 152
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x824faf75`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaFootstepAnalysisInfo {
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
    /// -   name:`"name"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(HkArrayRef<Primitive<char>>),
    /// # C++ Class Fields Info
    /// -   name:`"nameStrike"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameStrike")]
    NameStrike(HkArrayRef<Primitive<char>>),
    /// # C++ Class Fields Info
    /// -   name:`"nameLift"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameLift")]
    NameLift(HkArrayRef<Primitive<char>>),
    /// # C++ Class Fields Info
    /// -   name:`"nameLock"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameLock")]
    NameLock(HkArrayRef<Primitive<char>>),
    /// # C++ Class Fields Info
    /// -   name:`"nameUnlock"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameUnlock")]
    NameUnlock(HkArrayRef<Primitive<char>>),
    /// # C++ Class Fields Info
    /// -   name:`"minPos"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minPos")]
    MinPos(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"maxPos"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxPos")]
    MaxPos(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"minVel"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minVel")]
    MinVel(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVel"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVel")]
    MaxVel(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"allBonesDown"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allBonesDown")]
    AllBonesDown(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"anyBonesDown"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anyBonesDown")]
    AnyBonesDown(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"posTol"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "posTol")]
    PosTol(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"velTol"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velTol")]
    VelTol(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaFootstepAnalysisInfo, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(HkArrayRef<Primitive<char>>)),
    ("nameStrike" => NameStrike(HkArrayRef<Primitive<char>>)),
    ("nameLift" => NameLift(HkArrayRef<Primitive<char>>)),
    ("nameLock" => NameLock(HkArrayRef<Primitive<char>>)),
    ("nameUnlock" => NameUnlock(HkArrayRef<Primitive<char>>)),
    ("minPos" => MinPos(HkArrayRef<Primitive<f32>>)),
    ("maxPos" => MaxPos(HkArrayRef<Primitive<f32>>)),
    ("minVel" => MinVel(HkArrayRef<Primitive<f32>>)),
    ("maxVel" => MaxVel(HkArrayRef<Primitive<f32>>)),
    ("allBonesDown" => AllBonesDown(HkArrayRef<Primitive<f32>>)),
    ("anyBonesDown" => AnyBonesDown(HkArrayRef<Primitive<f32>>)),
    ("posTol" => PosTol(Primitive<f32>)),
    ("velTol" => VelTol(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
}
