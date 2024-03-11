//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMassProperties`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMassProperties<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMassProperties"`: The original C++ class name.
    #[serde(default = "HkpMassProperties::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x68a56834`: Unique value of this class.
    #[serde(default = "HkpMassProperties::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMassPropertiesHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMassPropertiesHkParam<'a>>
}

impl HkpMassProperties<'_> {
    /// Return `"hkpMassProperties"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpMassProperties".into()
    }

    /// Return `"0x68a56834"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x68a56834".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMassPropertiesHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"volume"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "volume")]
    Volume(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"centerOfMass"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMass")]
    CenterOfMass(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"inertiaTensor"`
    /// -   type: `hkMatrix3`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaTensor")]
    InertiaTensor(Matrix3<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMassPropertiesHkParam<'de>, "@name",
    ("volume" => Volume(Primitive<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("centerOfMass" => CenterOfMass(Vector4<f32>)),
    ("inertiaTensor" => InertiaTensor(Matrix3<f32>)),
}
