//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkUiAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkUiAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0xeb6e96e3`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkUiAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"hideInModeler"`
    /// -   type: `enum HideInModeler`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hideInModeler")]
    HideInModeler(Primitive<HideInModeler>),
    /// # C++ Class Fields Info
    /// -   name:`"label"`
    /// -   type: `char*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "label")]
    Label(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"group"`
    /// -   type: `char*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "group")]
    Group(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"hideBaseClassMembers"`
    /// -   type: `char*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hideBaseClassMembers")]
    HideBaseClassMembers(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"endGroup"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endGroup")]
    EndGroup(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"endGroup2"`
    /// -   type: `hkBool`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endGroup2")]
    EndGroup2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"advanced"`
    /// -   type: `hkBool`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "advanced")]
    Advanced(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkUiAttribute<'de>, "@name",
    ("visible" => Visible(Primitive<bool>)),
    ("hideInModeler" => HideInModeler(Primitive<HideInModeler>)),
    ("label" => Label(Primitive<Cow<'de, str>>)),
    ("group" => Group(Primitive<Cow<'de, str>>)),
    ("hideBaseClassMembers" => HideBaseClassMembers(Primitive<Cow<'de, str>>)),
    ("endGroup" => EndGroup(Primitive<bool>)),
    ("endGroup2" => EndGroup2(Primitive<bool>)),
    ("advanced" => Advanced(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HideInModeler {
    #[serde(rename = "NONE")]
    None = 0,
    #[serde(rename = "MAX")]
    Max = 1,
    #[serde(rename = "MAYA")]
    Maya = 2,
}
