//! A Rust structure that implements a serializer/deserializer corresponding to `hkbPoseMatchingGeneratorInternalState`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 28
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbPoseMatchingGeneratorInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbPoseMatchingGeneratorInternalState"`: The original C++ class name.
    #[serde(default = "HkbPoseMatchingGeneratorInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x552d9dd4`: Unique value of this class.
    #[serde(default = "HkbPoseMatchingGeneratorInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbPoseMatchingGeneratorInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbPoseMatchingGeneratorInternalStateHkParam<'a>>
}

impl HkbPoseMatchingGeneratorInternalState<'_> {
    /// Return `"hkbPoseMatchingGeneratorInternalState"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbPoseMatchingGeneratorInternalState".into()
    }

    /// Return `"0x552d9dd4"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x552d9dd4".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbPoseMatchingGeneratorInternalStateHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"currentMatch"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentMatch")]
    CurrentMatch(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"bestMatch"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bestMatch")]
    BestMatch(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"timeSinceBetterMatch"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeSinceBetterMatch")]
    TimeSinceBetterMatch(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "error")]
    Error(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"resetCurrentMatchLocalTime"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resetCurrentMatchLocalTime")]
    ResetCurrentMatchLocalTime(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbPoseMatchingGeneratorInternalStateHkParam<'de>, "@name",
    ("currentMatch" => CurrentMatch(Primitive<i32>)),
    ("bestMatch" => BestMatch(Primitive<i32>)),
    ("timeSinceBetterMatch" => TimeSinceBetterMatch(Primitive<f32>)),
    ("error" => Error(Primitive<f32>)),
    ("resetCurrentMatchLocalTime" => ResetCurrentMatchLocalTime(Primitive<bool>)),
}
