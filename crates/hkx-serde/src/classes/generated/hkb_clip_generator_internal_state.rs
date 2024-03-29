//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipGeneratorInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbClipGeneratorInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x26ce5bf3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipGeneratorInternalState {
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
    /// -   name:`"extractedMotion"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(Primitive<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"echos"`
    /// -   type: `hkArray<struct hkbClipGeneratorEcho>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echos")]
    Echos(HkArrayClass<HkbClipGeneratorEcho>),
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousUserControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousUserControlledTimeFraction")]
    PreviousUserControlledTimeFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bufferSize")]
    BufferSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"echoBufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoBufferSize")]
    EchoBufferSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"atEnd"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atEnd")]
    AtEnd(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"ignoreStartTime"`
    /// -   type: `hkBool`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreStartTime")]
    IgnoreStartTime(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"pingPongBackward"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pingPongBackward")]
    PingPongBackward(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorInternalState, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("extractedMotion" => ExtractedMotion(Primitive<QsTransform<f32>>)),
    ("echos" => Echos(HkArrayClass<HkbClipGeneratorEcho>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("time" => Time(Primitive<f32>)),
    ("previousUserControlledTimeFraction" => PreviousUserControlledTimeFraction(Primitive<f32>)),
    ("bufferSize" => BufferSize(Primitive<i32>)),
    ("echoBufferSize" => EchoBufferSize(Primitive<i32>)),
    ("atEnd" => AtEnd(Primitive<bool>)),
    ("ignoreStartTime" => IgnoreStartTime(Primitive<bool>)),
    ("pingPongBackward" => PingPongBackward(Primitive<bool>)),
}

impl ByteDeSerialize for HkbClipGeneratorInternalState {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
