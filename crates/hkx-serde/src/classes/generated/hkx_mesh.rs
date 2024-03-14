//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMesh`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxMesh`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xf2edcc5f`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMesh<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sections"`
    /// -   type: `hkArray&lt;hkxMeshSection*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sections")]
    Sections(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"userChannelInfos"`
    /// -   type: `hkArray&lt;hkxMeshUserChannelInfo*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userChannelInfos")]
    UserChannelInfos(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMesh<'de>, "@name",
    ("sections" => Sections(HkArrayRef<Cow<'de, str>>)),
    ("userChannelInfos" => UserChannelInfos(HkArrayRef<Cow<'de, str>>)),
}
