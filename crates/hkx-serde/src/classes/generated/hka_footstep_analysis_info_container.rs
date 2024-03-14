//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaFootstepAnalysisInfoContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaFootstepAnalysisInfoContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x1d81207c`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaFootstepAnalysisInfoContainer<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"previewInfo"`
    /// -   type: `hkArray&lt;hkaFootstepAnalysisInfo*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previewInfo")]
    PreviewInfo(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaFootstepAnalysisInfoContainer<'de>, "@name",
    ("previewInfo" => PreviewInfo(HkArrayRef<Cow<'de, str>>)),
}
