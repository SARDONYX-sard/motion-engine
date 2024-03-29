//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshotBlock`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkTrackerSerializableScanSnapshotBlock`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0xe7f23e6d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshotBlock {
    /// # C++ Class Fields Info
    /// -   name:`"typeIndex"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "typeIndex")]
    TypeIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"start"`
    /// -   type: `hkUlong`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "start")]
    Start(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"size"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "size")]
    Size(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"arraySize"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "arraySize")]
    ArraySize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"startReferenceIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startReferenceIndex")]
    StartReferenceIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numReferences"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numReferences")]
    NumReferences(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotBlock, "@name",
    ("typeIndex" => TypeIndex(Primitive<i32>)),
    ("start" => Start(Primitive<usize>)),
    ("size" => Size(Primitive<usize>)),
    ("arraySize" => ArraySize(Primitive<i32>)),
    ("startReferenceIndex" => StartReferenceIndex(Primitive<i32>)),
    ("numReferences" => NumReferences(Primitive<i32>)),
}

impl ByteDeSerialize for HkTrackerSerializableScanSnapshotBlock {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
