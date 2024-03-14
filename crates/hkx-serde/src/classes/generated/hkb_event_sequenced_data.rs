//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventSequencedData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbEventSequencedData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkbSequencedData`/`0xda8c7d7d`
/// - signature: `0x76798eb8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventSequencedData {
    /// # C++ Class Fields Info
    /// -   name:`"events"`
    /// -   type: `hkArray&lt;struct hkbEventSequencedDataSequencedEvent&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "events")]
    Events(HkArrayClass<HkbEventSequencedDataSequencedEvent>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventSequencedData, "@name",
    ("events" => Events(HkArrayClass<HkbEventSequencedDataSequencedEvent>)),
}
