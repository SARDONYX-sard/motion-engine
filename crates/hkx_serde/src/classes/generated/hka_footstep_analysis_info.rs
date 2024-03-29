//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaFootstepAnalysisInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaFootstepAnalysisInfo {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(HkArrayRef<char>),
    /// # C++ Class Fields Info
    /// -   name:`"nameStrike"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameStrike")]
    NameStrike(HkArrayRef<char>),
    /// # C++ Class Fields Info
    /// -   name:`"nameLift"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameLift")]
    NameLift(HkArrayRef<char>),
    /// # C++ Class Fields Info
    /// -   name:`"nameLock"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameLock")]
    NameLock(HkArrayRef<char>),
    /// # C++ Class Fields Info
    /// -   name:`"nameUnlock"`
    /// -   type: `hkArray<hkChar>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameUnlock")]
    NameUnlock(HkArrayRef<char>),
    /// # C++ Class Fields Info
    /// -   name:`"minPos"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minPos")]
    MinPos(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxPos"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxPos")]
    MaxPos(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minVel"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minVel")]
    MinVel(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVel"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVel")]
    MaxVel(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"allBonesDown"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allBonesDown")]
    AllBonesDown(HkArrayNum<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"anyBonesDown"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anyBonesDown")]
    AnyBonesDown(HkArrayNum<f32>),
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
    ("name" => Name(HkArrayRef<char>)),
    ("nameStrike" => NameStrike(HkArrayRef<char>)),
    ("nameLift" => NameLift(HkArrayRef<char>)),
    ("nameLock" => NameLock(HkArrayRef<char>)),
    ("nameUnlock" => NameUnlock(HkArrayRef<char>)),
    ("minPos" => MinPos(HkArrayNum<f32>)),
    ("maxPos" => MaxPos(HkArrayNum<f32>)),
    ("minVel" => MinVel(HkArrayNum<f32>)),
    ("maxVel" => MaxVel(HkArrayNum<f32>)),
    ("allBonesDown" => AllBonesDown(HkArrayNum<f32>)),
    ("anyBonesDown" => AnyBonesDown(HkArrayNum<f32>)),
    ("posTol" => PosTol(Primitive<f32>)),
    ("velTol" => VelTol(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
}

impl ByteDeSerialize for HkaFootstepAnalysisInfo {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
