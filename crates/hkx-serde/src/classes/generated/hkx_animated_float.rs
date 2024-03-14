//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxAnimatedFloat`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxAnimatedFloat`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xce8b2fbd`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAnimatedFloat {
    /// # C++ Class Fields Info
    /// -   name:`"floats"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floats")]
    Floats(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"hint"`
    /// -   type: `enum Hint`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hint")]
    Hint(Hint),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAnimatedFloat, "@name",
    ("floats" => Floats(HkArrayRef<Primitive<f32>>)),
    ("hint" => Hint(Hint)),
}
