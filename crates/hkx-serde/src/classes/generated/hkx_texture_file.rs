//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxTextureFile`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxTextureFile`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x1e289259`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxTextureFile<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"filename"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "filename")]
    Filename(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"originalFilename"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalFilename")]
    OriginalFilename(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxTextureFile<'de>, "@name",
    ("filename" => Filename(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("originalFilename" => OriginalFilename(Primitive<Cow<'de, str>>)),
}
