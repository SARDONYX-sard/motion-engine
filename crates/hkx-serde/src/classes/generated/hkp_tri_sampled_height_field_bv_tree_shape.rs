//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriSampledHeightFieldBvTreeShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriSampledHeightFieldBvTreeShape {
    /// # C++ Class Fields Info
    /// -   name:`"childContainer"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childContainer")]
    ChildContainer(HkpSingleShapeContainer),
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
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
    Padding([Primitive<u8>; 12]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldBvTreeShape, "@name",
    ("childContainer" => ChildContainer(HkpSingleShapeContainer)),
    ("childSize" => ChildSize(Primitive<i32>)),
    ("wantAabbRejectionTest" => WantAabbRejectionTest(Primitive<bool>)),
    ("padding" => Padding([Primitive<u8>; 12])),
}
