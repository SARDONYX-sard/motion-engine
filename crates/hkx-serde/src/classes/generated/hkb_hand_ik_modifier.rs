//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbHandIkModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xef8bc2f7`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkModifier {
    /// # C++ Class Fields Info
    /// -   name:`"hands"`
    /// -   type: `hkArray&lt;struct hkbHandIkModifierHand&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hands")]
    Hands(HkArrayClass<HkbHandIkModifierHand>),
    /// # C++ Class Fields Info
    /// -   name:`"fadeInOutCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeInOutCurve")]
    FadeInOutCurve(Primitive<BlendCurve>),
    /// # C++ Class Fields Info
    /// -   name:`"internalHandData"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalHandData", skip_serializing)]
    InternalHandData(HkArrayRef<Primitive<()>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkModifier, "@name",
    ("hands" => Hands(HkArrayClass<HkbHandIkModifierHand>)),
    ("fadeInOutCurve" => FadeInOutCurve(Primitive<BlendCurve>)),
    ("internalHandData" => InternalHandData(HkArrayRef<Primitive<()>>)),
}
