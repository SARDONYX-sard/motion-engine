//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventRangeData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbEventRangeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x6cb92c76`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventRangeData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"upperBound"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upperBound")]
    UpperBound(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "event")]
    Event(SingleClass<HkbEventProperty<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventRangeMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventRangeMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventRangeData<'de>, "@name",
    ("upperBound" => UpperBound(Primitive<f32>)),
    ("event" => Event(SingleClass<HkbEventProperty<'de>>)),
    ("eventMode" => EventMode(Primitive<EventRangeMode>)),
}

impl ByteDeSerialize for HkbEventRangeData<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum EventRangeMode {
    #[serde(rename = "EVENT_MODE_SEND_ON_ENTER_RANGE")]
    EventModeSendOnEnterRange = 0,
    #[serde(rename = "EVENT_MODE_SEND_WHEN_IN_RANGE")]
    EventModeSendWhenInRange = 1,
}
