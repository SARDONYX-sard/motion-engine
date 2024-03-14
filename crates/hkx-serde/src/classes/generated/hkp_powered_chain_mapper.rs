//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainMapper`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPoweredChainMapper`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x7a77ef5`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainMapper<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"links"`
    /// -   type: `hkArray&lt;struct hkpPoweredChainMapperLinkInfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "links")]
    Links(HkArrayClass<HkpPoweredChainMapperLinkInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"targets"`
    /// -   type: `hkArray&lt;struct hkpPoweredChainMapperTarget&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targets")]
    Targets(HkArrayClass<HkpPoweredChainMapperTarget>),
    /// # C++ Class Fields Info
    /// -   name:`"chains"`
    /// -   type: `hkArray&lt;hkpConstraintChainInstance*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chains")]
    Chains(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapper<'de>, "@name",
    ("links" => Links(HkArrayClass<HkpPoweredChainMapperLinkInfo>)),
    ("targets" => Targets(HkArrayClass<HkpPoweredChainMapperTarget>)),
    ("chains" => Chains(HkArrayRef<Cow<'de, str>>)),
}
