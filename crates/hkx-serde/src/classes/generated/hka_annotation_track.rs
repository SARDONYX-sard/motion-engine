//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaAnnotationTrack`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaAnnotationTrack`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xd4114fdd`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnnotationTrack<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"trackName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "trackName", default)]
    TrackName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"annotations"`
    /// -   type: `hkArray&lt;struct hkaAnnotationTrackAnnotation&gt;`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotations", default)]
    Annotations(HkArrayClass<HkaAnnotationTrackAnnotation>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnnotationTrack<'de>, "@name",
    ("trackName" => TrackName(Primitive<Cow<'de, str>>)),
    ("annotations" => Annotations(HkArrayClass<HkaAnnotationTrackAnnotation>)),
}
