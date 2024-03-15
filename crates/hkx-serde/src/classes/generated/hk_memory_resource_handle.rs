//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryResourceHandle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMemoryResourceHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkResourceHandle`/`0x4e94146`
/// - signature: `0xbffac086`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryResourceHandle<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"variant"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variant")]
    Variant(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"references"`
    /// -   type: `hkArray&lt;struct hkMemoryResourceHandleExternalLink&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "references")]
    References(HkArrayClass<HkMemoryResourceHandleExternalLink>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceHandle<'de>, "@name",
    ("variant" => Variant(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("references" => References(HkArrayClass<HkMemoryResourceHandleExternalLink>)),
}
