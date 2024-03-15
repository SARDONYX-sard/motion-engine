//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpEntitySpuCollisionCallback`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpEntitySpuCollisionCallback`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x81147f05`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpEntitySpuCollisionCallback<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"util"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "util", skip_serializing)]
    Util(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"capacity"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "capacity", skip_serializing)]
    Capacity(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"eventFilter"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventFilter")]
    EventFilter(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"userFilter"`
    /// -   type: `hkUint8`
    /// - offset: 7
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userFilter")]
    UserFilter(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntitySpuCollisionCallback<'de>, "@name",
    ("util" => Util(Primitive<Cow<'de, str>>)),
    ("capacity" => Capacity(Primitive<u16>)),
    ("eventFilter" => EventFilter(Primitive<u8>)),
    ("userFilter" => UserFilter(Primitive<u8>)),
}
