//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCollidableBoundingVolumeData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpCollidableBoundingVolumeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: false
/// - signature: `0xb5f0e6b1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCollidableBoundingVolumeData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "min")]
    Min(CStyleArray<[u32; 3]>),
    /// # C++ Class Fields Info
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMin")]
    ExpansionMin(CStyleArray<[u8; 3]>),
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
    Max(CStyleArray<[u32; 3]>),
    /// # C++ Class Fields Info
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMax")]
    ExpansionMax(CStyleArray<[u8; 3]>),
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
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "numChildShapeAabbs", skip_serializing)]
    NumChildShapeAabbs(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"capacityChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "capacityChildShapeAabbs", skip_serializing)]
    CapacityChildShapeAabbs(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeAabbs"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "childShapeAabbs", skip_serializing)]
    ChildShapeAabbs(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeKeys"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "childShapeKeys", skip_serializing)]
    ChildShapeKeys(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidableBoundingVolumeData<'de>, "@name",
    ("min" => Min(CStyleArray<[u32; 3]>)),
    ("expansionMin" => ExpansionMin(CStyleArray<[u8; 3]>)),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max(CStyleArray<[u32; 3]>)),
    ("expansionMax" => ExpansionMax(CStyleArray<[u8; 3]>)),
    ("padding" => Padding(Primitive<u8>)),
    ("numChildShapeAabbs" => NumChildShapeAabbs(Primitive<u16>)),
    ("capacityChildShapeAabbs" => CapacityChildShapeAabbs(Primitive<u16>)),
    ("childShapeAabbs" => ChildShapeAabbs(Primitive<Cow<'de, str>>)),
    ("childShapeKeys" => ChildShapeKeys(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkpCollidableBoundingVolumeData<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
