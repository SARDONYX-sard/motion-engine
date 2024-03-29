//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshotAllocation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkTrackerSerializableScanSnapshotAllocation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x9ab3a6ac`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshotAllocation {
    /// # C++ Class Fields Info
    /// -   name:`"start"`
    /// -   type: `hkUlong`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "start")]
    Start(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"size"`
    /// -   type: `hkUlong`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "size")]
    Size(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"traceId"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceId")]
    TraceId(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotAllocation, "@name",
    ("start" => Start(Primitive<usize>)),
    ("size" => Size(Primitive<usize>)),
    ("traceId" => TraceId(Primitive<i32>)),
}

impl ByteDeSerialize for HkTrackerSerializableScanSnapshotAllocation {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
