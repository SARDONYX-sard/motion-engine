//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbNamedRealEventPayload`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbNamedRealEventPayload`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: true
/// -    parent: `hkbNamedEventPayload`/`0x65bdd3a0`
/// - signature: `0x9c99fd70`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbNamedRealEventPayload {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbNamedRealEventPayload, "@name",
    ("data" => Data(Primitive<f32>)),
}
