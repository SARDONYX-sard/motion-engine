//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConstraintCollisionFilter`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkpPairCollisionFilter`/`0x4abc140e`
/// - signature: `0xc3b577b1`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintCollisionFilter {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintCollisionFilter, "@name",
}
