//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMeshVertexBuffer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMeshVertexBuffer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x534b08c8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshVertexBuffer {
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshVertexBuffer, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Flags {
    #[serde(rename = "ACCESS_READ")]
    AccessRead = 1,
    #[serde(rename = "ACCESS_WRITE")]
    AccessWrite = 2,
    #[serde(rename = "ACCESS_READ_WRITE")]
    AccessReadWrite = 3,
    #[serde(rename = "ACCESS_WRITE_DISCARD")]
    AccessWriteDiscard = 4,
    #[serde(rename = "ACCESS_ELEMENT_ARRAY")]
    AccessElementArray = 8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LockResult {
    #[serde(rename = "RESULT_FAILURE")]
    ResultFailure = 0,
    #[serde(rename = "RESULT_SUCCESS")]
    ResultSuccess = 1,
    #[serde(rename = "RESULT_IN_USE")]
    ResultInUse = 2,
}
