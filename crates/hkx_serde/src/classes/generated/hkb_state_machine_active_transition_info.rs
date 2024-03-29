//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineActiveTransitionInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbStateMachineActiveTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0xbb90d54f`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineActiveTransitionInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transitionEffect"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "transitionEffect", skip_serializing)]
    TransitionEffect(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionEffectInternalStateInfo"`
    /// -   type: `struct hkbNodeInternalStateInfo*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionEffectInternalStateInfo")]
    TransitionEffectInternalStateInfo(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReference"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReference")]
    TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionInfoReferenceForTE"`
    /// -   type: `struct hkbStateMachineTransitionInfoReference`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionInfoReferenceForTE")]
    TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>),
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
    ("transitionEffect" => TransitionEffect(Primitive<Cow<'de, str>>)),
    ("transitionEffectInternalStateInfo" => TransitionEffectInternalStateInfo(Primitive<Cow<'de, str>>)),
    ("transitionInfoReference" => TransitionInfoReference(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("transitionInfoReferenceForTE" => TransitionInfoReferenceForTe(SingleClass<HkbStateMachineTransitionInfoReference>)),
    ("fromStateId" => FromStateId(Primitive<i32>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
    ("isReturnToPreviousState" => IsReturnToPreviousState(Primitive<bool>)),
}

impl ByteDeSerialize for HkbStateMachineActiveTransitionInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
