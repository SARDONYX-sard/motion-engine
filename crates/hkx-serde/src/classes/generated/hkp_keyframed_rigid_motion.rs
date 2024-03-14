//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpKeyframedRigidMotion`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpKeyframedRigidMotion`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 288
/// -    vtable: true
/// -    parent: `hkpMotion`/`0x98aadb4f`
/// - signature: `0xbafa2bb7`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpKeyframedRigidMotion {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpKeyframedRigidMotion, "@name",
}
