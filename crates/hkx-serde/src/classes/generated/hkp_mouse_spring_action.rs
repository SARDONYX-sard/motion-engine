//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMouseSpringAction`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkpUnaryAction/`895532c0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMouseSpringAction<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMouseSpringAction"`: The original C++ class name.
    #[serde(default = "HkpMouseSpringAction::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6e087fd6`: Unique value of this class.
    #[serde(default = "HkpMouseSpringAction::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMouseSpringActionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMouseSpringActionHkParam<'a>>
}

impl HkpMouseSpringAction<'_> {
    /// Return `"hkpMouseSpringAction"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpMouseSpringAction".into()
    }

    /// Return `"0x6e087fd6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6e087fd6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMouseSpringActionHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"positionInRbLocal"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionInRbLocal")]
    PositionInRbLocal(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"mousePositionInWorld"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mousePositionInWorld")]
    MousePositionInWorld(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"springDamping"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springDamping")]
    SpringDamping(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"springElasticity"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springElasticity")]
    SpringElasticity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxRelativeForce"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxRelativeForce")]
    MaxRelativeForce(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"objectDamping"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectDamping")]
    ObjectDamping(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapeKey")]
    ShapeKey(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"applyCallbacks"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "applyCallbacks", skip_serializing)]
    ApplyCallbacks(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMouseSpringActionHkParam<'de>, "@name",
    ("positionInRbLocal" => PositionInRbLocal(Vector4<f32>)),
    ("mousePositionInWorld" => MousePositionInWorld(Vector4<f32>)),
    ("springDamping" => SpringDamping(Primitive<f32>)),
    ("springElasticity" => SpringElasticity(Primitive<f32>)),
    ("maxRelativeForce" => MaxRelativeForce(Primitive<f32>)),
    ("objectDamping" => ObjectDamping(Primitive<f32>)),
    ("shapeKey" => ShapeKey(Primitive<u32>)),
    ("applyCallbacks" => ApplyCallbacks(Vec<()>)),
}
