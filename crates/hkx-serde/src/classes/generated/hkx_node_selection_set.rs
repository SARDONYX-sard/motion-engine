//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxNodeSelectionSet`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxNodeSelectionSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0xd753fc4d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxNodeSelectionSet<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"selectedNodes"`
    /// -   type: `hkArray&lt;hkxNode*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectedNodes")]
    SelectedNodes(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxNodeSelectionSet<'de>, "@name",
    ("selectedNodes" => SelectedNodes(HkArrayRef<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
