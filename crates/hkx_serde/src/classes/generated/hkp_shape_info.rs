//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpShapeInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpShapeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xea7f1d08`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpShapeInfo<'a> {
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
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"isHierarchicalCompound"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isHierarchicalCompound")]
    IsHierarchicalCompound(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"hkdShapesCollected"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hkdShapesCollected")]
    HkdShapesCollected(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShapeNames")]
    ChildShapeNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"childTransforms"`
    /// -   type: `hkArray<hkTransform>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childTransforms")]
    ChildTransforms(HkArrayMatrix4<Transform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpShapeInfo<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("isHierarchicalCompound" => IsHierarchicalCompound(Primitive<bool>)),
    ("hkdShapesCollected" => HkdShapesCollected(Primitive<bool>)),
    ("childShapeNames" => ChildShapeNames(HkArrayStringPtr<'de>)),
    ("childTransforms" => ChildTransforms(HkArrayMatrix4<Transform<f32>>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
}

impl ByteDeSerialize for HkpShapeInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
