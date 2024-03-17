//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallSocketChainData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBallSocketChainData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkpConstraintChainData`/`0x5facc7ff`
/// - signature: `0x102aae9c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallSocketChainData<'a> {
    // C++ Parent class(`hkpConstraintChainData` => parent: `hkpConstraintData`) has no fields

    /// # C++ Parent class(`hkpConstraintData` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),

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
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"infos"`
    /// -   type: `hkArray&lt;struct hkpBallSocketChainDataConstraintInfo&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "infos")]
    Infos(HkArrayClass<HkpBallSocketChainDataConstraintInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cfm"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfm")]
    Cfm(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxErrorDistance"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxErrorDistance")]
    MaxErrorDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallSocketChainData<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("atoms" => Atoms(HkpBridgeAtoms<'de>)),
    ("infos" => Infos(HkArrayClass<HkpBallSocketChainDataConstraintInfo>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("cfm" => Cfm(Primitive<f32>)),
    ("maxErrorDistance" => MaxErrorDistance(Primitive<f32>)),
}
