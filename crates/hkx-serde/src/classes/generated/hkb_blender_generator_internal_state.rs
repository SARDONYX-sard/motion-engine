//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlenderGeneratorInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbBlenderGeneratorInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x84717488`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlenderGeneratorInternalState {
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
    /// -   name:`"childrenInternalStates"`
    /// -   type: `hkArray<struct hkbBlenderGeneratorChildInternalState>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childrenInternalStates")]
    ChildrenInternalStates(HkArrayClass<HkbBlenderGeneratorChildInternalState>),
    /// # C++ Class Fields Info
    /// -   name:`"sortedChildren"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sortedChildren")]
    SortedChildren(HkArrayNum<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endIntervalWeight"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endIntervalWeight")]
    EndIntervalWeight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numActiveChildren"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numActiveChildren")]
    NumActiveChildren(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"beginIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "beginIntervalIndex")]
    BeginIntervalIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"endIntervalIndex"`
    /// -   type: `hkInt16`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endIntervalIndex")]
    EndIntervalIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"initSync"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initSync")]
    InitSync(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"doSubtractiveBlend"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "doSubtractiveBlend")]
    DoSubtractiveBlend(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlenderGeneratorInternalState, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("childrenInternalStates" => ChildrenInternalStates(HkArrayClass<HkbBlenderGeneratorChildInternalState>)),
    ("sortedChildren" => SortedChildren(HkArrayNum<i16>)),
    ("endIntervalWeight" => EndIntervalWeight(Primitive<f32>)),
    ("numActiveChildren" => NumActiveChildren(Primitive<i32>)),
    ("beginIntervalIndex" => BeginIntervalIndex(Primitive<i16>)),
    ("endIntervalIndex" => EndIntervalIndex(Primitive<i16>)),
    ("initSync" => InitSync(Primitive<bool>)),
    ("doSubtractiveBlend" => DoSubtractiveBlend(Primitive<bool>)),
}
