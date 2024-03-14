//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConvexTransformShapeBase`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexTransformShapeBase`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0xfbd72f9`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexTransformShapeBase {
    /// # C++ Class Fields Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape")]
    ChildShape(HkpSingleShapeContainer),
    /// # C++ Class Fields Info
    /// -   name:`"childShapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeSize", skip_serializing)]
    ChildShapeSize(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexTransformShapeBase, "@name",
    ("childShape" => ChildShape(HkpSingleShapeContainer)),
    ("childShapeSize" => ChildShapeSize(Primitive<i32>)),
}
