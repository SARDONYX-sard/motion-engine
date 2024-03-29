//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineProspectiveTransitionInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbStateMachineProspectiveTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x3ab09a2e`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineProspectiveTransitionInfo {
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// # C++ Class Fields Info
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineProspectiveTransitionInfo, "@name",
    ("transitionInfoReference" => TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
}

impl ByteDeSerialize for HkbStateMachineProspectiveTransitionInfo {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
