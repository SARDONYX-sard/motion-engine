//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandIkControlsModifierHand`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 96
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandIkControlsModifierHand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandIkControlsModifierHand"`: Name of this class.
    #[serde(default = "HkbHandIkControlsModifierHand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9c72e9e3`: Unique value of this class.
    #[serde(default = "HkbHandIkControlsModifierHand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandIkControlsModifierHandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandIkControlsModifierHandHkParam<'a>>
}

impl HkbHandIkControlsModifierHand<'_> {
    /// Return `"hkbHandIkControlsModifierHand"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbHandIkControlsModifierHand".into()
    }

    /// Return `"0x9c72e9e3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9c72e9e3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkControlsModifierHandHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"controlData"`
    /// -   type: `struct hkbHandIkControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbHandIkControlData),
    /// # Information on fields in the original C++ class
    /// -   name:`"handIndex"`
    /// -   type: `hkInt32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handIndex")]
    HandIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkControlsModifierHandHkParam<'de>, "@name",
    ("controlData" => ControlData(HkbHandIkControlData)),
    ("handIndex" => HandIndex(i32)),
    ("enable" => Enable(bool)),
}