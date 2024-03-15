//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkAabbHalf`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkAabbHalf`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x1d716a17`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAabbHalf {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkUint16[6]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data([Primitive<u16>; 6]),
    /// # C++ Class Fields Info
    /// -   name:`"extras"`
    /// -   type: `hkUint16[2]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extras")]
    Extras([Primitive<u16>; 2]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAabbHalf, "@name",
    ("data" => Data([Primitive<u16>; 6])),
    ("extras" => Extras([Primitive<u16>; 2])),
}
