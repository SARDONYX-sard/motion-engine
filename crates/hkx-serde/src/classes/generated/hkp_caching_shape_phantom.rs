//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCachingShapePhantom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpCachingShapePhantom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 368
/// -    vtable: true
/// -    parent: `hkpShapePhantom`/`0xcb22fbcd`
/// - signature: `0xcf227f58`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCachingShapePhantom {
    /// # C++ Class Fields Info
    /// -   name:`"collisionDetails"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 352
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionDetails", skip_serializing)]
    CollisionDetails(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 364
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "orderDirty", skip_serializing)]
    OrderDirty(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCachingShapePhantom, "@name",
    ("collisionDetails" => CollisionDetails(HkArrayRef<()>)),
    ("orderDirty" => OrderDirty(Primitive<bool>)),
}
