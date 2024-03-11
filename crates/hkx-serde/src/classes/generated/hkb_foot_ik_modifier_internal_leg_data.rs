//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkModifierInternalLegData`, a class defined in C++
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
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbFootIkModifierInternalLegData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkModifierInternalLegData"`: The original C++ class name.
    #[serde(default = "HkbFootIkModifierInternalLegData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe5ca3677`: Unique value of this class.
    #[serde(default = "HkbFootIkModifierInternalLegData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkModifierInternalLegDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkModifierInternalLegDataHkParam<'a>>
}

impl HkbFootIkModifierInternalLegData<'_> {
    /// Return `"hkbFootIkModifierInternalLegData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbFootIkModifierInternalLegData".into()
    }

    /// Return `"0xe5ca3677"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe5ca3677".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifierInternalLegDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"groundPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundPosition")]
    GroundPosition(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"footIkSolver"`
    /// -   type: `void*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "footIkSolver", skip_serializing)]
    FootIkSolver(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierInternalLegDataHkParam<'de>, "@name",
    ("groundPosition" => GroundPosition(Vector4<f32>)),
    ("footIkSolver" => FootIkSolver(())),
}
