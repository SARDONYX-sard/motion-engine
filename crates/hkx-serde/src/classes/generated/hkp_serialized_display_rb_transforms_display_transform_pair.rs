//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSerializedDisplayRbTransformsDisplayTransformPair`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSerializedDisplayRbTransformsDisplayTransformPair`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x94ac5bec`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedDisplayRbTransformsDisplayTransformPair<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"rb"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rb")]
    Rb(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"localToDisplay"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localToDisplay")]
    LocalToDisplay(Transform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedDisplayRbTransformsDisplayTransformPair<'de>, "@name",
    ("rb" => Rb(Cow<'de, str>)),
    ("localToDisplay" => LocalToDisplay(Transform<f32>)),
}
