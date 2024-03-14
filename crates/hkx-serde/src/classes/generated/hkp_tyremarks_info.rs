//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpTyremarksInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTyremarksInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3d0433d6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTyremarksInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"minTyremarkEnergy"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minTyremarkEnergy")]
    MinTyremarkEnergy(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxTyremarkEnergy"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxTyremarkEnergy")]
    MaxTyremarkEnergy(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"tyremarksWheel"`
    /// -   type: `hkArray&lt;hkpTyremarksWheel*&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tyremarksWheel")]
    TyremarksWheel(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarksInfo<'de>, "@name",
    ("minTyremarkEnergy" => MinTyremarkEnergy(Primitive<f32>)),
    ("maxTyremarkEnergy" => MaxTyremarkEnergy(Primitive<f32>)),
    ("tyremarksWheel" => TyremarksWheel(HkArrayRef<Cow<'de, str>>)),
}
