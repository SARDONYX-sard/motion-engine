//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWeldingUtility`, a class defined in C++
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
/// -    size: 1
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpWeldingUtility<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpWeldingUtility"`: Name of this class.
    #[serde(default = "HkpWeldingUtility::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb2b41feb`: Unique value of this class.
    #[serde(default = "HkpWeldingUtility::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpWeldingUtilityHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpWeldingUtilityHkParam<'a>>
}

impl HkpWeldingUtility<'_> {
    /// Return `"hkpWeldingUtility"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpWeldingUtility".into()
    }

    /// Return `"0xb2b41feb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb2b41feb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWeldingUtilityHkParam<'a> {
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWeldingUtilityHkParam<'de>, "@name",
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WeldingType {
    #[serde(rename = "WELDING_TYPE_ANTICLOCKWISE")]
    WeldingTypeAnticlockwise = 0,
    #[serde(rename = "WELDING_TYPE_CLOCKWISE")]
    WeldingTypeClockwise = 4,
    #[serde(rename = "WELDING_TYPE_TWO_SIDED")]
    WeldingTypeTwoSided = 5,
    #[serde(rename = "WELDING_TYPE_NONE")]
    WeldingTypeNone = 6,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SectorType {
    #[serde(rename = "ACCEPT_0")]
    Accept0 = 1,
    #[serde(rename = "SNAP_0")]
    Snap0 = 0,
    #[serde(rename = "REJECT")]
    Reject = 2,
    #[serde(rename = "SNAP_1")]
    Snap1 = 4,
    #[serde(rename = "ACCEPT_1")]
    Accept1 = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NumAngles {
    #[serde(rename = "NUM_ANGLES")]
    NumAngles = 31,
}