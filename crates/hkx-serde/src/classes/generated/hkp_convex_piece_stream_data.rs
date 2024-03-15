//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexPieceStreamData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexPieceStreamData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa5bd1d6e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexPieceStreamData {
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

    /// # C++ Class Fields Info
    /// -   name:`"convexPieceStream"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieceStream")]
    ConvexPieceStream(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"convexPieceOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieceOffsets")]
    ConvexPieceOffsets(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"convexPieceSingleTriangles"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieceSingleTriangles")]
    ConvexPieceSingleTriangles(HkArrayRef<Primitive<u32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexPieceStreamData, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("convexPieceStream" => ConvexPieceStream(HkArrayRef<Primitive<u32>>)),
    ("convexPieceOffsets" => ConvexPieceOffsets(HkArrayRef<Primitive<u32>>)),
    ("convexPieceSingleTriangles" => ConvexPieceSingleTriangles(HkArrayRef<Primitive<u32>>)),
}
