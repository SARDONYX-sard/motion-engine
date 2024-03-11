//! A Rust structure that implements a serializer/deserializer corresponding to `BSInterpValueModifier`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsInterpValueModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSInterpValueModifier"`: The original C++ class name.
    #[serde(default = "BsInterpValueModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x29adc802`: Unique value of this class.
    #[serde(default = "BsInterpValueModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsInterpValueModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsInterpValueModifierHkParam<'a>>
}

impl BsInterpValueModifier<'_> {
    /// Return `"BSInterpValueModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSInterpValueModifier".into()
    }

    /// Return `"0x29adc802"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x29adc802".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsInterpValueModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"source"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "source")]
    Source(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"target"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "target")]
    Target(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"result"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "result")]
    Result(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsInterpValueModifierHkParam<'de>, "@name",
    ("source" => Source(Primitive<f32>)),
    ("target" => Target(Primitive<f32>)),
    ("result" => Result(Primitive<f32>)),
    ("gain" => Gain(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
}
