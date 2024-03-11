//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSetupStabilizationAtom`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSetupStabilizationAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSetupStabilizationAtom"`: The original C++ class name.
    #[serde(default = "HkpSetupStabilizationAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf05d137e`: Unique value of this class.
    #[serde(default = "HkpSetupStabilizationAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSetupStabilizationAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSetupStabilizationAtomHkParam<'a>>
}

impl HkpSetupStabilizationAtom<'_> {
    /// Return `"hkpSetupStabilizationAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSetupStabilizationAtom".into()
    }

    /// Return `"0xf05d137e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf05d137e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetupStabilizationAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"enabled"`
    /// -   type: `hkBool`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enabled")]
    Enabled(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"padding"`
    /// -   type: `hkUint8[8]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding([Primitive<u8>; 8]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetupStabilizationAtomHkParam<'de>, "@name",
    ("enabled" => Enabled(Primitive<bool>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("padding" => Padding([Primitive<u8>; 8])),
}
