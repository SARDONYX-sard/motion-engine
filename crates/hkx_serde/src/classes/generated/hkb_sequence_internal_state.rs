//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSequenceInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbSequenceInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x419b9a05`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSequenceInternalState {
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
    /// -   name:`"nextSampleEvents"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleEvents")]
    NextSampleEvents(HkArrayNum<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleReals"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleReals")]
    NextSampleReals(HkArrayNum<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleBools"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleBools")]
    NextSampleBools(HkArrayNum<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"nextSampleInts"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextSampleInts")]
    NextSampleInts(HkArrayNum<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSequenceInternalState, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("nextSampleEvents" => NextSampleEvents(HkArrayNum<i32>)),
    ("nextSampleReals" => NextSampleReals(HkArrayNum<i32>)),
    ("nextSampleBools" => NextSampleBools(HkArrayNum<i32>)),
    ("nextSampleInts" => NextSampleInts(HkArrayNum<i32>)),
    ("time" => Time(Primitive<f32>)),
    ("isEnabled" => IsEnabled(Primitive<bool>)),
}

impl ByteDeSerialize for HkbSequenceInternalState {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
