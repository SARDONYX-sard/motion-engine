//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineDelayedTransitionInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbStateMachineDelayedTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0x26d5499`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineDelayedTransitionInfo {
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransition"`
    /// -   type: `struct hkbStateMachineProspectiveTransitionInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayedTransition")]
    DelayedTransition(SingleClass<HkbStateMachineProspectiveTransitionInfo>),
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
    ("delayedTransition" => DelayedTransition(SingleClass<HkbStateMachineProspectiveTransitionInfo>)),
    ("timeDelayed" => TimeDelayed(Primitive<f32>)),
    ("isDelayedTransitionReturnToPreviousState" => IsDelayedTransitionReturnToPreviousState(Primitive<bool>)),
    ("wasInAbutRangeLastFrame" => WasInAbutRangeLastFrame(Primitive<bool>)),
}

impl ByteDeSerialize for HkbStateMachineDelayedTransitionInfo {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
