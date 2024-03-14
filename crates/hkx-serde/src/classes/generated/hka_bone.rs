//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaBone`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaBone`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x35912f8a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaBone<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"lockTranslation"`
    /// -   type: `hkBool`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockTranslation")]
    LockTranslation(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaBone<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("lockTranslation" => LockTranslation(Primitive<bool>)),
}
