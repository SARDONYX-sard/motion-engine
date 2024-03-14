//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxAnimatedQuaternion`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxAnimatedQuaternion`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xb4f01baa`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAnimatedQuaternion {
    /// # C++ Class Fields Info
    /// -   name:`"quaternions"`
    /// -   type: `hkArray&lt;hkQuaternion&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quaternions")]
    Quaternions(HkArrayVector<Quaternion<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAnimatedQuaternion, "@name",
    ("quaternions" => Quaternions(HkArrayVector<Quaternion<f32>>)),
}
