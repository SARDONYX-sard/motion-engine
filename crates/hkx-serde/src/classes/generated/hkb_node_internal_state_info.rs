//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbNodeInternalStateInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbNodeInternalStateInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 100
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x7db9971d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbNodeInternalStateInfo<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"syncInfo"`
    /// -   type: `struct hkbGeneratorSyncInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncInfo")]
    SyncInfo(HkbGeneratorSyncInfo),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"internalState"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalState")]
    InternalState(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeId"`
    /// -   type: `hkInt16`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeId")]
    NodeId(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"hasActivateBeenCalled"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hasActivateBeenCalled")]
    HasActivateBeenCalled(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbNodeInternalStateInfo<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("syncInfo" => SyncInfo(HkbGeneratorSyncInfo)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("internalState" => InternalState(Primitive<Cow<'de, str>>)),
    ("nodeId" => NodeId(Primitive<i16>)),
    ("hasActivateBeenCalled" => HasActivateBeenCalled(Primitive<bool>)),
}
