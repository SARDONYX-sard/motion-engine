//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriSampledHeightFieldBvTreeShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpTriSampledHeightFieldBvTreeShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpBvTreeShape`/`0xa823d623`
/// - signature: `0x58e1e585`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriSampledHeightFieldBvTreeShape<'a> {
    /// # C++ Parent class(`hkpBvTreeShape` => parent: `hkpShape`) field Info
    /// -   name:`"bvTreeType"`
    /// -   type: `enum BvTreeType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bvTreeType")]
    BvTreeType(Primitive<BvTreeType>),

    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

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
    /// -   name:`"childContainer"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childContainer")]
    ChildContainer(SingleClass<HkpSingleShapeContainer<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"wantAabbRejectionTest"`
    /// -   type: `hkBool`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantAabbRejectionTest")]
    WantAabbRejectionTest(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkUint8[12]`
    /// - offset: 33
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(CStyleArray<[u8; 12]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldBvTreeShape<'de>, "@name",
    ("bvTreeType" => BvTreeType(Primitive<BvTreeType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("childContainer" => ChildContainer(SingleClass<HkpSingleShapeContainer<'de>>)),
    ("childSize" => ChildSize(Primitive<i32>)),
    ("wantAabbRejectionTest" => WantAabbRejectionTest(Primitive<bool>)),
    ("padding" => Padding(CStyleArray<[u8; 12]>)),
}

impl ByteDeSerialize for HkpTriSampledHeightFieldBvTreeShape<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
