//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshot`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkTrackerSerializableScanSnapshot`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 92
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x875af1d9`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshot {
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"allocations"`
    /// -   type: `hkArray&lt;struct hkTrackerSerializableScanSnapshotAllocation&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allocations", default)]
    Allocations(HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>),
    /// # C++ Class Fields Info
    /// -   name:`"blocks"`
    /// -   type: `hkArray&lt;struct hkTrackerSerializableScanSnapshotBlock&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blocks", default)]
    Blocks(HkArrayClass<HkTrackerSerializableScanSnapshotBlock>),
    /// # C++ Class Fields Info
    /// -   name:`"refs"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refs", default)]
    Refs(HkArrayRef<Primitive<i32>>),
    /// # C++ Class Fields Info
    /// -   name:`"typeNames"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "typeNames", default)]
    TypeNames(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"traceText"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceText", default)]
    TraceText(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"traceAddrs"`
    /// -   type: `hkArray&lt;hkUint64&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceAddrs", default)]
    TraceAddrs(HkArrayRef<Primitive<u64>>),
    /// # C++ Class Fields Info
    /// -   name:`"traceParents"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceParents", default)]
    TraceParents(HkArrayRef<Primitive<i32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshot, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("allocations" => Allocations(HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>)),
    ("blocks" => Blocks(HkArrayClass<HkTrackerSerializableScanSnapshotBlock>)),
    ("refs" => Refs(HkArrayRef<Primitive<i32>>)),
    ("typeNames" => TypeNames(HkArrayRef<Primitive<u8>>)),
    ("traceText" => TraceText(HkArrayRef<Primitive<u8>>)),
    ("traceAddrs" => TraceAddrs(HkArrayRef<Primitive<u64>>)),
    ("traceParents" => TraceParents(HkArrayRef<Primitive<i32>>)),
}
