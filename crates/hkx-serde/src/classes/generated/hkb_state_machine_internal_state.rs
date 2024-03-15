//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xbd1a7502`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineInternalState {
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
    /// -   name:`"activeTransitions"`
    /// -   type: `hkArray&lt;struct hkbStateMachineActiveTransitionInfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeTransitions")]
    ActiveTransitions(HkArrayClass<HkbStateMachineActiveTransitionInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionFlags"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionFlags")]
    TransitionFlags(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"wildcardTransitionFlags"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wildcardTransitionFlags")]
    WildcardTransitionFlags(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransitions"`
    /// -   type: `hkArray&lt;struct hkbStateMachineDelayedTransitionInfo&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayedTransitions")]
    DelayedTransitions(HkArrayClass<HkbStateMachineDelayedTransitionInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"timeInState"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeInState")]
    TimeInState(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"lastLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastLocalTime")]
    LastLocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentStateId"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentStateId")]
    CurrentStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousStateId"`
    /// -   type: `hkInt32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousStateId")]
    PreviousStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"nextStartStateIndexOverride"`
    /// -   type: `hkInt32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextStartStateIndexOverride")]
    NextStartStateIndexOverride(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateOrTransitionChanged")]
    StateOrTransitionChanged(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"echoNextUpdate"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoNextUpdate")]
    EchoNextUpdate(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineInternalState, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("activeTransitions" => ActiveTransitions(HkArrayClass<HkbStateMachineActiveTransitionInfo>)),
    ("transitionFlags" => TransitionFlags(HkArrayRef<Primitive<u8>>)),
    ("wildcardTransitionFlags" => WildcardTransitionFlags(HkArrayRef<Primitive<u8>>)),
    ("delayedTransitions" => DelayedTransitions(HkArrayClass<HkbStateMachineDelayedTransitionInfo>)),
    ("timeInState" => TimeInState(Primitive<f32>)),
    ("lastLocalTime" => LastLocalTime(Primitive<f32>)),
    ("currentStateId" => CurrentStateId(Primitive<i32>)),
    ("previousStateId" => PreviousStateId(Primitive<i32>)),
    ("nextStartStateIndexOverride" => NextStartStateIndexOverride(Primitive<i32>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
    ("echoNextUpdate" => EchoNextUpdate(Primitive<bool>)),
}
