//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRootLevelContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkRootLevelContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x2772c11e`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkRootLevelContainer {
    /// # C++ Class Fields Info
    /// -   name:`"namedVariants"`
    /// -   type: `hkArray&lt;struct hkRootLevelContainerNamedVariant&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedVariants", default)]
    NamedVariants(HkArrayClass<HkRootLevelContainerNamedVariant>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRootLevelContainer, "@name",
    ("namedVariants" => NamedVariants(HkArrayClass<HkRootLevelContainerNamedVariant>)),
}
