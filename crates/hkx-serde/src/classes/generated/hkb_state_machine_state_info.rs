//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineStateInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineStateInfo<'a> {
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

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
    /// -   name:`"listeners"`
    /// -   type: `hkArray<hkbStateListener*>`
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
    EnterNotifyEvents(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"exitNotifyEvents"`
    /// -   type: `struct hkbStateMachineEventPropertyArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitNotifyEvents")]
    ExitNotifyEvents(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"transitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitions")]
    Transitions(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generator")]
    Generator(Primitive<Cow<'a, str>>),
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
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("enterNotifyEvents" => EnterNotifyEvents(Primitive<Cow<'de, str>>)),
    ("exitNotifyEvents" => ExitNotifyEvents(Primitive<Cow<'de, str>>)),
    ("transitions" => Transitions(Primitive<Cow<'de, str>>)),
    ("generator" => Generator(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("stateId" => StateId(Primitive<i32>)),
    ("probability" => Probability(Primitive<f32>)),
    ("enable" => Enable(Primitive<bool>)),
}

impl ByteDeSerialize for HkbStateMachineStateInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
