//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbHandIkDriverInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbHandIkDriverInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc299090a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkDriverInfo {
    /// # C++ Class Fields Info
    /// -   name:`"hands"`
    /// -   type: `hkArray&lt;struct hkbHandIkDriverInfoHand&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hands")]
    Hands(HkArrayClass<HkbHandIkDriverInfoHand>),
    /// # C++ Class Fields Info
    /// -   name:`"fadeInOutCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeInOutCurve")]
    FadeInOutCurve(BlendCurve),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkDriverInfo, "@name",
    ("hands" => Hands(HkArrayClass<HkbHandIkDriverInfoHand>)),
    ("fadeInOutCurve" => FadeInOutCurve(BlendCurve)),
}
