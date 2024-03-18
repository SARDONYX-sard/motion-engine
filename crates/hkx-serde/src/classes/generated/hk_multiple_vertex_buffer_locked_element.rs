//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultipleVertexBufferLockedElement`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkMultipleVertexBufferLockedElement`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 7
/// -    vtable: false
/// - signature: `0xa0e22afc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferLockedElement {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBufferIndex")]
    VertexBufferIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"elementIndex"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementIndex")]
    ElementIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"lockedBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockedBufferIndex")]
    LockedBufferIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"vertexFormatIndex"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexFormatIndex")]
    VertexFormatIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"lockFlags"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockFlags")]
    LockFlags(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"outputBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "outputBufferIndex")]
    OutputBufferIndex(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"emulatedIndex"`
    /// -   type: `hkInt8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "emulatedIndex")]
    EmulatedIndex(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferLockedElement, "@name",
    ("vertexBufferIndex" => VertexBufferIndex(Primitive<u8>)),
    ("elementIndex" => ElementIndex(Primitive<u8>)),
    ("lockedBufferIndex" => LockedBufferIndex(Primitive<u8>)),
    ("vertexFormatIndex" => VertexFormatIndex(Primitive<u8>)),
    ("lockFlags" => LockFlags(Primitive<u8>)),
    ("outputBufferIndex" => OutputBufferIndex(Primitive<u8>)),
    ("emulatedIndex" => EmulatedIndex(Primitive<i8>)),
}
