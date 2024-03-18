//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventSequencedDataSequencedEvent`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbEventSequencedDataSequencedEvent`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x9139b821`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventSequencedDataSequencedEvent<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEvent`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(SingleClass<HkbEvent<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventSequencedDataSequencedEvent<'de>, "@name",
    ("event" => Event(SingleClass<HkbEvent<'de>>)),
    ("time" => Time(Primitive<f32>)),
}
