//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSequence`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSequence`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 168
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x43182ca3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSequence<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"eventSequencedData"`
    /// -   type: `hkArray&lt;hkbEventSequencedData*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventSequencedData")]
    EventSequencedData(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"realVariableSequencedData"`
    /// -   type: `hkArray&lt;hkbRealVariableSequencedData*&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "realVariableSequencedData")]
    RealVariableSequencedData(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"boolVariableSequencedData"`
    /// -   type: `hkArray&lt;hkbBoolVariableSequencedData*&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boolVariableSequencedData")]
    BoolVariableSequencedData(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"intVariableSequencedData"`
    /// -   type: `hkArray&lt;hkbIntVariableSequencedData*&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "intVariableSequencedData")]
    IntVariableSequencedData(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"enableEventId"`
    /// -   type: `hkInt32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableEventId")]
    EnableEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"disableEventId"`
    /// -   type: `hkInt32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disableEventId")]
    DisableEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbSequenceStringData*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableIdMap", skip_serializing)]
    VariableIdMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleEvents"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleEvents", skip_serializing)]
    NextSampleEvents(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleReals"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleReals", skip_serializing)]
    NextSampleReals(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleBools"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleBools", skip_serializing)]
    NextSampleBools(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleInts"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextSampleInts", skip_serializing)]
    NextSampleInts(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isEnabled", skip_serializing)]
    IsEnabled(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSequence<'de>, "@name",
    ("eventSequencedData" => EventSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("realVariableSequencedData" => RealVariableSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("boolVariableSequencedData" => BoolVariableSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("intVariableSequencedData" => IntVariableSequencedData(HkArrayRef<Cow<'de, str>>)),
    ("enableEventId" => EnableEventId(Primitive<i32>)),
    ("disableEventId" => DisableEventId(Primitive<i32>)),
    ("stringData" => StringData(Cow<'de, str>)),
    ("variableIdMap" => VariableIdMap(Cow<'de, str>)),
    ("eventIdMap" => EventIdMap(Cow<'de, str>)),
    ("nextSampleEvents" => NextSampleEvents(HkArrayRef<()>)),
    ("nextSampleReals" => NextSampleReals(HkArrayRef<()>)),
    ("nextSampleBools" => NextSampleBools(HkArrayRef<()>)),
    ("nextSampleInts" => NextSampleInts(HkArrayRef<()>)),
    ("time" => Time(Primitive<f32>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
}
