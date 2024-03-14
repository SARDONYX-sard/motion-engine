//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxMeshUserChannelInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxMeshUserChannelInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0x270724a5`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMeshUserChannelInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"className"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "className")]
    ClassName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMeshUserChannelInfo<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("className" => ClassName(Primitive<Cow<'de, str>>)),
}
