//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventRaisedInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbEventRaisedInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc02da3`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventRaisedInfo<'a> {
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

    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"eventName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventName")]
    EventName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"raisedBySdk"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raisedBySdk")]
    RaisedBySdk(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"senderId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "senderId")]
    SenderId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventRaisedInfo<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("eventName" => EventName(Primitive<Cow<'de, str>>)),
    ("raisedBySdk" => RaisedBySdk(Primitive<bool>)),
    ("senderId" => SenderId(Primitive<i32>)),
    ("padding" => Padding(Primitive<i32>)),
}
