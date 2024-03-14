//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpBvShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBvShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0x286eb64c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBvShape<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"boundingVolumeShape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boundingVolumeShape")]
    BoundingVolumeShape(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape")]
    ChildShape(HkpSingleShapeContainer),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBvShape<'de>, "@name",
    ("boundingVolumeShape" => BoundingVolumeShape(Cow<'de, str>)),
    ("childShape" => ChildShape(HkpSingleShapeContainer)),
}
