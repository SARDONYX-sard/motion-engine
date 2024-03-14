//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSIsActiveModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSIsActiveModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xb0fde45a`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsIsActiveModifier {
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive0"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive0")]
    BIsActive0(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive0"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive0")]
    BInvertActive0(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive1"`
    /// -   type: `hkBool`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive1")]
    BIsActive1(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive1"`
    /// -   type: `hkBool`
    /// - offset: 47
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive1")]
    BInvertActive1(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive2"`
    /// -   type: `hkBool`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive2")]
    BIsActive2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive2"`
    /// -   type: `hkBool`
    /// - offset: 49
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive2")]
    BInvertActive2(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive3"`
    /// -   type: `hkBool`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive3")]
    BIsActive3(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive3"`
    /// -   type: `hkBool`
    /// - offset: 51
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive3")]
    BInvertActive3(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bIsActive4"`
    /// -   type: `hkBool`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive4")]
    BIsActive4(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"bInvertActive4"`
    /// -   type: `hkBool`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive4")]
    BInvertActive4(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsIsActiveModifier, "@name",
    ("bIsActive0" => BIsActive0(Primitive<bool>)),
    ("bInvertActive0" => BInvertActive0(Primitive<bool>)),
    ("bIsActive1" => BIsActive1(Primitive<bool>)),
    ("bInvertActive1" => BInvertActive1(Primitive<bool>)),
    ("bIsActive2" => BIsActive2(Primitive<bool>)),
    ("bInvertActive2" => BInvertActive2(Primitive<bool>)),
    ("bIsActive3" => BIsActive3(Primitive<bool>)),
    ("bInvertActive3" => BInvertActive3(Primitive<bool>)),
    ("bIsActive4" => BIsActive4(Primitive<bool>)),
    ("bInvertActive4" => BInvertActive4(Primitive<bool>)),
}
