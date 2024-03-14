//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetLocalTransformsConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSetLocalTransformsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x6e2a5198`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalTransformsConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"transformA"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformA")]
    TransformA(Transform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"transformB"`
    /// -   type: `hkTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformB")]
    TransformB(Transform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalTransformsConstraintAtom, "@name",
    ("transformA" => TransformA(Transform<f32>)),
    ("transformB" => TransformB(Transform<f32>)),
}
