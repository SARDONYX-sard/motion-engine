//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaInterleavedUncompressedAnimation`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaInterleavedUncompressedAnimation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkaAnimation`/`0xa6fa7e88`
/// - signature: `0x930af031`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaInterleavedUncompressedAnimation {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkArrayVector<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"floats"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floats")]
    Floats(HkArrayRef<Primitive<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaInterleavedUncompressedAnimation, "@name",
    ("transforms" => Transforms(HkArrayVector<QsTransform<f32>>)),
    ("floats" => Floats(HkArrayRef<Primitive<f32>>)),
}
