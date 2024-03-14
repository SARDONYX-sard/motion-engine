//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxTextureInplace`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxTextureInplace`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd45841d6`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxTextureInplace<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"fileType"`
    /// -   type: `hkChar[4]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fileType")]
    FileType([Primitive<char>; 4]),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"originalFilename"`
    /// -   type: `hkStringPtr`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalFilename")]
    OriginalFilename(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxTextureInplace<'de>, "@name",
    ("fileType" => FileType([Primitive<char>; 4])),
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("originalFilename" => OriginalFilename(Primitive<Cow<'de, str>>)),
}
