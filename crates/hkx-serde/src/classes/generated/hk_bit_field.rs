//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkBitField`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkBitField`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xda41bd9b`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkBitField {
    /// # C++ Class Fields Info
    /// -   name:`"words"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "words")]
    Words(HkArrayRef<Primitive<u32>>),
    /// # C++ Class Fields Info
    /// -   name:`"numBits"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBits")]
    NumBits(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkBitField, "@name",
    ("words" => Words(HkArrayRef<Primitive<u32>>)),
    ("numBits" => NumBits(Primitive<i32>)),
}
