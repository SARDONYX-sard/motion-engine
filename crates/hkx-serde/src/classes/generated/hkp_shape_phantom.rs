//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpShapePhantom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpShapePhantom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 352
/// -    vtable: true
/// -    parent: `hkpPhantom`/`0x9b7e6f86`
/// - signature: `0xcb22fbcd`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpShapePhantom {
    /// # C++ Class Fields Info
    /// -   name:`"motionState"`
    /// -   type: `struct hkMotionState`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionState")]
    MotionState(HkMotionState),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpShapePhantom, "@name",
    ("motionState" => MotionState(HkMotionState)),
}
