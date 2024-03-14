//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbStateMachineActiveTransitionInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineActiveTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xbb90d54f`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineActiveTransitionInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transitionEffect"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "transitionEffect", skip_serializing)]
    TransitionEffect(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionEffectInternalStateInfo"`
    /// -   type: `struct hkbNodeInternalStateInfo*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionEffectInternalStateInfo")]
    TransitionEffectInternalStateInfo(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(HkbStateMachineTransitionInfoReference),
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(HkbStateMachineTransitionInfoReference),
    /// # C++ Class Fields Info
    /// -   name:`"fromStateId"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fromStateId")]
    FromStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"isReturnToPreviousState"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isReturnToPreviousState")]
    IsReturnToPreviousState(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineActiveTransitionInfo<'de>, "@name",
    ("transitionEffect" => TransitionEffect(Cow<'de, str>)),
    ("transitionEffectInternalStateInfo" => TransitionEffectInternalStateInfo(Cow<'de, str>)),
    ("transitionInfoReference" => TransitionInfoReference(HkbStateMachineTransitionInfoReference)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(HkbStateMachineTransitionInfoReference)),
    ("fromStateId" => FromStateId(Primitive<i32>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
    ("isReturnToPreviousState" => IsReturnToPreviousState(Primitive<bool>)),
}
