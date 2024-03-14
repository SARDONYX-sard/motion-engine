//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineStateInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineStateInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbBindable`/`0x2c1432d7`
/// - signature: `0xed7f9d0`
/// -   version: 4
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineStateInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"listeners"`
    /// -   type: `hkArray&lt;hkbStateListener*&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "listeners")]
    Listeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"enterNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enterNotifyEvents")]
    EnterNotifyEvents(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"exitNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitNotifyEvents")]
    ExitNotifyEvents(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"transitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitions")]
    Transitions(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generator")]
    Generator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"stateId"`
    /// -   type: `hkInt32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateId")]
    StateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"probability"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "probability")]
    Probability(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineStateInfo<'de>, "@name",
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("enterNotifyEvents" => EnterNotifyEvents(Cow<'de, str>)),
    ("exitNotifyEvents" => ExitNotifyEvents(Cow<'de, str>)),
    ("transitions" => Transitions(Cow<'de, str>)),
    ("generator" => Generator(Cow<'de, str>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("stateId" => StateId(Primitive<i32>)),
    ("probability" => Probability(Primitive<f32>)),
    ("enable" => Enable(Primitive<bool>)),
}
