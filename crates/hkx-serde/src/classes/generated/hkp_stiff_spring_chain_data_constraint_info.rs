//! A Rust structure that implements a serializer/deserializer corresponding to `hkpStiffSpringChainDataConstraintInfo`, a class defined in C++
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
/// -    size: 48
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpStiffSpringChainDataConstraintInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpStiffSpringChainDataConstraintInfo"`: The original C++ class name.
    #[serde(default = "HkpStiffSpringChainDataConstraintInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc624a180`: Unique value of this class.
    #[serde(default = "HkpStiffSpringChainDataConstraintInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpStiffSpringChainDataConstraintInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpStiffSpringChainDataConstraintInfoHkParam<'a>>
}

impl HkpStiffSpringChainDataConstraintInfo<'_> {
    /// Return `"hkpStiffSpringChainDataConstraintInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpStiffSpringChainDataConstraintInfo".into()
    }

    /// Return `"0xc624a180"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc624a180".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStiffSpringChainDataConstraintInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"pivotInA"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInA")]
    PivotInA(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"pivotInB"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInB")]
    PivotInB(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"springLength"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springLength")]
    SpringLength(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpStiffSpringChainDataConstraintInfoHkParam<'de>, "@name",
    ("pivotInA" => PivotInA(Vector4<f32>)),
    ("pivotInB" => PivotInB(Vector4<f32>)),
    ("springLength" => SpringLength(Primitive<f32>)),
}
