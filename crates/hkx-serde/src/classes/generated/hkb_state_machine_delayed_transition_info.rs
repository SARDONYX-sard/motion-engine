//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineDelayedTransitionInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineDelayedTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x26d5499`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineDelayedTransitionInfo {
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransition"`
    /// -   type: `struct hkbStateMachineProspectiveTransitionInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayedTransition")]
    DelayedTransition(HkbStateMachineProspectiveTransitionInfo),
    /// # C++ Class Fields Info
    /// -   name:`"timeDelayed"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeDelayed")]
    TimeDelayed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isDelayedTransitionReturnToPreviousState"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isDelayedTransitionReturnToPreviousState")]
    IsDelayedTransitionReturnToPreviousState(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"wasInAbutRangeLastFrame"`
    /// -   type: `hkBool`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wasInAbutRangeLastFrame")]
    WasInAbutRangeLastFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineDelayedTransitionInfo, "@name",
    ("delayedTransition" => DelayedTransition(HkbStateMachineProspectiveTransitionInfo)),
    ("timeDelayed" => TimeDelayed(Primitive<f32>)),
    ("isDelayedTransitionReturnToPreviousState" => IsDelayedTransitionReturnToPreviousState(Primitive<bool>)),
    ("wasInAbutRangeLastFrame" => WasInAbutRangeLastFrame(Primitive<bool>)),
}
