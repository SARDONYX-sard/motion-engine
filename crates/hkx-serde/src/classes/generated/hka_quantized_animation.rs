//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaQuantizedAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaQuantizedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x3920f053`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaQuantizedAnimation<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"endian"`
    /// -   type: `hkUint32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endian")]
    Endian(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "skeleton", skip_serializing)]
    Skeleton(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaQuantizedAnimation<'de>, "@name",
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
    ("endian" => Endian(Primitive<u32>)),
    ("skeleton" => Skeleton(Cow<'de, str>)),
}
