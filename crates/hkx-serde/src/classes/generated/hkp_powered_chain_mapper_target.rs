//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpPoweredChainMapperTarget`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPoweredChainMapperTarget`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xf651c74d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainMapperTarget<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"chain"`
    /// -   type: `struct hkpPoweredChainData*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chain")]
    Chain(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"infoIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "infoIndex")]
    InfoIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperTarget<'de>, "@name",
    ("chain" => Chain(Cow<'de, str>)),
    ("infoIndex" => InfoIndex(Primitive<i32>)),
}
