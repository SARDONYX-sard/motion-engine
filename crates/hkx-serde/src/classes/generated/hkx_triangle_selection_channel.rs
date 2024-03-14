//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxTriangleSelectionChannel`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxTriangleSelectionChannel`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa02cfca9`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxTriangleSelectionChannel {
    /// # C++ Class Fields Info
    /// -   name:`"selectedTriangles"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectedTriangles")]
    SelectedTriangles(HkArrayRef<Primitive<i32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxTriangleSelectionChannel, "@name",
    ("selectedTriangles" => SelectedTriangles(HkArrayRef<Primitive<i32>>)),
}
