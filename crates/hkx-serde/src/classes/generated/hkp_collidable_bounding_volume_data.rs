//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCollidableBoundingVolumeData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCollidableBoundingVolumeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xb5f0e6b1`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCollidableBoundingVolumeData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "min")]
    Min([Primitive<u32>; 3]),
    /// # C++ Class Fields Info
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMin")]
    ExpansionMin([Primitive<u8>; 3]),
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
    Max([Primitive<u32>; 3]),
    /// # C++ Class Fields Info
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMax")]
    ExpansionMax([Primitive<u8>; 3]),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkUint8`
    /// - offset: 31
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"numChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numChildShapeAabbs", skip_serializing)]
    NumChildShapeAabbs(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"capacityChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "capacityChildShapeAabbs", skip_serializing)]
    CapacityChildShapeAabbs(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeAabbs"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeAabbs", skip_serializing)]
    ChildShapeAabbs(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeKeys"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeKeys", skip_serializing)]
    ChildShapeKeys(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidableBoundingVolumeData<'de>, "@name",
    ("min" => Min([Primitive<u32>; 3])),
    ("expansionMin" => ExpansionMin([Primitive<u8>; 3])),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max([Primitive<u32>; 3])),
    ("expansionMax" => ExpansionMax([Primitive<u8>; 3])),
    ("padding" => Padding(Primitive<u8>)),
    ("numChildShapeAabbs" => NumChildShapeAabbs(Primitive<u16>)),
    ("capacityChildShapeAabbs" => CapacityChildShapeAabbs(Primitive<u16>)),
    ("childShapeAabbs" => ChildShapeAabbs(Primitive<Cow<'de, str>>)),
    ("childShapeKeys" => ChildShapeKeys(Primitive<Cow<'de, str>>)),
}
