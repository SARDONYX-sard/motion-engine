//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkAabbUint32`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkAabbUint32`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x11e7c11`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAabbUint32 {
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "min")]
    Min(CStyleArray<u32, 3>),
    /// # C++ Class Fields Info
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMin")]
    ExpansionMin(CStyleArray<u8, 3>),
    /// # C++ Class Fields Info
    /// -   name:`"expansionShift"`
    /// -   type: `hkUint8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionShift")]
    ExpansionShift(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"max"`
    /// -   type: `hkUint32[3]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "max")]
    Max(CStyleArray<u32, 3>),
    /// # C++ Class Fields Info
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMax")]
    ExpansionMax(CStyleArray<u8, 3>),
    /// # C++ Class Fields Info
    /// -   name:`"shapeKeyByte"`
    /// -   type: `hkUint8`
    /// - offset: 31
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapeKeyByte")]
    ShapeKeyByte(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAabbUint32, "@name",
    ("min" => Min(CStyleArray<u32, 3>)),
    ("expansionMin" => ExpansionMin(CStyleArray<u8, 3>)),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max(CStyleArray<u32, 3>)),
    ("expansionMax" => ExpansionMax(CStyleArray<u8, 3>)),
    ("shapeKeyByte" => ShapeKeyByte(Primitive<u8>)),
}
