//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxAnimatedMatrix`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxAnimatedMatrix`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5838e337`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAnimatedMatrix {
    /// # C++ Class Fields Info
    /// -   name:`"matrices"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matrices")]
    Matrices(HkArrayVector<Matrix4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"hint"`
    /// -   type: `enum Hint`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hint")]
    Hint(Primitive<Hint>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAnimatedMatrix, "@name",
    ("matrices" => Matrices(HkArrayVector<Matrix4<f32>>)),
    ("hint" => Hint(Primitive<Hint>)),
}
