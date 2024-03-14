//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTransitionEffect`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x945da157`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTransitionEffect {
    /// # C++ Class Fields Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<SelfTransitionMode>),
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventMode>),
    /// # C++ Class Fields Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Primitive<Unknown>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTransitionEffect, "@name",
    ("selfTransitionMode" => SelfTransitionMode(Primitive<SelfTransitionMode>)),
    ("eventMode" => EventMode(Primitive<EventMode>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<Unknown>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelfTransitionMode {
    #[serde(rename = "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC")]
    SelfTransitionModeContinueIfCyclicBlendIfAcyclic = 0,
    #[serde(rename = "SELF_TRANSITION_MODE_CONTINUE")]
    SelfTransitionModeContinue = 1,
    #[serde(rename = "SELF_TRANSITION_MODE_RESET")]
    SelfTransitionModeReset = 2,
    #[serde(rename = "SELF_TRANSITION_MODE_BLEND")]
    SelfTransitionModeBlend = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventMode {
    #[serde(rename = "EVENT_MODE_DEFAULT")]
    EventModeDefault = 0,
    #[serde(rename = "EVENT_MODE_PROCESS_ALL")]
    EventModeProcessAll = 1,
    #[serde(rename = "EVENT_MODE_IGNORE_FROM_GENERATOR")]
    EventModeIgnoreFromGenerator = 2,
    #[serde(rename = "EVENT_MODE_IGNORE_TO_GENERATOR")]
    EventModeIgnoreToGenerator = 3,
}
