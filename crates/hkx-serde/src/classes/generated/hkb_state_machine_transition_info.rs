//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineTransitionInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xcdec8025`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTransitionInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"triggerInterval"`
    /// -   type: `struct hkbStateMachineTimeInterval`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerInterval")]
    TriggerInterval(HkbStateMachineTimeInterval),
    /// # C++ Class Fields Info
    /// -   name:`"initiateInterval"`
    /// -   type: `struct hkbStateMachineTimeInterval`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initiateInterval")]
    InitiateInterval(HkbStateMachineTimeInterval),
    /// # C++ Class Fields Info
    /// -   name:`"transition"`
    /// -   type: `struct hkbTransitionEffect*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transition")]
    Transition(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"condition"`
    /// -   type: `struct hkbCondition*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "condition")]
    Condition(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"eventId"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventId")]
    EventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"fromNestedStateId"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fromNestedStateId")]
    FromNestedStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"toNestedStateId"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toNestedStateId")]
    ToNestedStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"priority"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "priority")]
    Priority(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags TransitionFlags`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<TransitionFlags>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfo<'de>, "@name",
    ("triggerInterval" => TriggerInterval(HkbStateMachineTimeInterval)),
    ("initiateInterval" => InitiateInterval(HkbStateMachineTimeInterval)),
    ("transition" => Transition(Cow<'de, str>)),
    ("condition" => Condition(Cow<'de, str>)),
    ("eventId" => EventId(Primitive<i32>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
    ("fromNestedStateId" => FromNestedStateId(Primitive<i32>)),
    ("toNestedStateId" => ToNestedStateId(Primitive<i32>)),
    ("priority" => Priority(Primitive<i16>)),
    ("flags" => Flags(Primitive<TransitionFlags>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionFlags {
    #[serde(rename = "FLAG_USE_TRIGGER_INTERVAL")]
    FlagUseTriggerInterval = 1,
    #[serde(rename = "FLAG_USE_INITIATE_INTERVAL")]
    FlagUseInitiateInterval = 2,
    #[serde(rename = "FLAG_UNINTERRUPTIBLE_WHILE_PLAYING")]
    FlagUninterruptibleWhilePlaying = 4,
    #[serde(rename = "FLAG_UNINTERRUPTIBLE_WHILE_DELAYED")]
    FlagUninterruptibleWhileDelayed = 8,
    #[serde(rename = "FLAG_DELAY_STATE_CHANGE")]
    FlagDelayStateChange = 16,
    #[serde(rename = "FLAG_DISABLED")]
    FlagDisabled = 32,
    #[serde(rename = "FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE")]
    FlagDisallowReturnToPreviousState = 64,
    #[serde(rename = "FLAG_DISALLOW_RANDOM_TRANSITION")]
    FlagDisallowRandomTransition = 128,
    #[serde(rename = "FLAG_DISABLE_CONDITION")]
    FlagDisableCondition = 256,
    #[serde(rename = "FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE")]
    FlagAllowSelfTransitionByTransitionFromAnyState = 512,
    #[serde(rename = "FLAG_IS_GLOBAL_WILDCARD")]
    FlagIsGlobalWildcard = 1024,
    #[serde(rename = "FLAG_IS_LOCAL_WILDCARD")]
    FlagIsLocalWildcard = 2048,
    #[serde(rename = "FLAG_FROM_NESTED_STATE_ID_IS_VALID")]
    FlagFromNestedStateIdIsValid = 4096,
    #[serde(rename = "FLAG_TO_NESTED_STATE_ID_IS_VALID")]
    FlagToNestedStateIdIsValid = 8192,
    #[serde(rename = "FLAG_ABUT_AT_END_OF_FROM_GENERATOR")]
    FlagAbutAtEndOfFromGenerator = 16384,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InternalFlagBits {
    #[serde(rename = "FLAG_INTERNAL_IN_TRIGGER_INTERVAL")]
    FlagInternalInTriggerInterval = 1,
    #[serde(rename = "FLAG_INTERNAL_IN_INITIATE_INTERVAL")]
    FlagInternalInInitiateInterval = 2,
}
