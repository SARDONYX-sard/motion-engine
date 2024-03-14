//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbEventsFromRangeModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventsFromRangeModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xbc561b6e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventsFromRangeModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"inputValue"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inputValue")]
    InputValue(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"lowerBound"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lowerBound")]
    LowerBound(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"eventRanges"`
    /// -   type: `struct hkbEventRangeDataArray*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventRanges")]
    EventRanges(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"wasActiveInPreviousFrame"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "wasActiveInPreviousFrame", skip_serializing)]
    WasActiveInPreviousFrame(HkArrayRef<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventsFromRangeModifier<'de>, "@name",
    ("inputValue" => InputValue(Primitive<f32>)),
    ("lowerBound" => LowerBound(Primitive<f32>)),
    ("eventRanges" => EventRanges(Cow<'de, str>)),
    ("wasActiveInPreviousFrame" => WasActiveInPreviousFrame(HkArrayRef<()>)),
}
