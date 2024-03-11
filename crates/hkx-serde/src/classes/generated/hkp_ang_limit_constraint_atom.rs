//! A Rust structure that implements a serializer/deserializer corresponding to `hkpAngLimitConstraintAtom`, a class defined in C++
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
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpAngLimitConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpAngLimitConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpAngLimitConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9be0d9d`: Unique value of this class.
    #[serde(default = "HkpAngLimitConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpAngLimitConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpAngLimitConstraintAtomHkParam<'a>>
}

impl HkpAngLimitConstraintAtom<'_> {
    /// Return `"hkpAngLimitConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpAngLimitConstraintAtom".into()
    }

    /// Return `"0x9be0d9d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9be0d9d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngLimitConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAxis")]
    LimitAxis(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"minAngle"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAngle")]
    MinAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"angularLimitsTauFactor"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularLimitsTauFactor")]
    AngularLimitsTauFactor(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngLimitConstraintAtomHkParam<'de>, "@name",
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("limitAxis" => LimitAxis(Primitive<u8>)),
    ("minAngle" => MinAngle(Primitive<f32>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(Primitive<f32>)),
}
