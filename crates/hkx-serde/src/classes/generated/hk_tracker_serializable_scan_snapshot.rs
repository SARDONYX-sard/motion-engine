//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshot`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshot {
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
    /// -   name:`"allocations"`
    /// -   type: `hkArray<struct hkTrackerSerializableScanSnapshotAllocation>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allocations")]
    Allocations(HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>),
    /// # C++ Class Fields Info
    /// -   name:`"blocks"`
    /// -   type: `hkArray<struct hkTrackerSerializableScanSnapshotBlock>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blocks")]
    Blocks(HkArrayClass<HkTrackerSerializableScanSnapshotBlock>),
    /// # C++ Class Fields Info
    /// -   name:`"refs"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refs")]
    Refs(HkArrayNum<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"typeNames"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "typeNames")]
    TypeNames(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"traceText"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceText")]
    TraceText(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"traceAddrs"`
    /// -   type: `hkArray<hkUint64>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceAddrs")]
    TraceAddrs(HkArrayNum<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"traceParents"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceParents")]
    TraceParents(HkArrayNum<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshot, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("allocations" => Allocations(HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>)),
    ("blocks" => Blocks(HkArrayClass<HkTrackerSerializableScanSnapshotBlock>)),
    ("refs" => Refs(HkArrayNum<i32>)),
    ("typeNames" => TypeNames(HkArrayNum<u8>)),
    ("traceText" => TraceText(HkArrayNum<u8>)),
    ("traceAddrs" => TraceAddrs(HkArrayNum<u64>)),
    ("traceParents" => TraceParents(HkArrayNum<i32>)),
}

impl ByteDeSerialize for HkTrackerSerializableScanSnapshot {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
