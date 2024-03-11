//! A Rust structure that implements a serializer/deserializer corresponding to `hkpGravityGun`, a class defined in C++
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
/// -  parent: hkpFirstPersonGun/`852ab70b`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpGravityGun<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpGravityGun"`: The original C++ class name.
    #[serde(default = "HkpGravityGun::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5e2754cd`: Unique value of this class.
    #[serde(default = "HkpGravityGun::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpGravityGunHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpGravityGunHkParam<'a>>
}

impl HkpGravityGun<'_> {
    /// Return `"hkpGravityGun"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpGravityGun".into()
    }

    /// Return `"0x5e2754cd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5e2754cd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGravityGunHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"grabbedBodies"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "grabbedBodies", skip_serializing)]
    GrabbedBodies(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"maxNumObjectsPicked"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumObjectsPicked")]
    MaxNumObjectsPicked(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxMassOfObjectPicked"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxMassOfObjectPicked")]
    MaxMassOfObjectPicked(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxDistOfObjectPicked"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxDistOfObjectPicked")]
    MaxDistOfObjectPicked(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"impulseAppliedWhenObjectNotPicked"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "impulseAppliedWhenObjectNotPicked")]
    ImpulseAppliedWhenObjectNotPicked(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"throwVelocity"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "throwVelocity")]
    ThrowVelocity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"capturedObjectPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capturedObjectPosition")]
    CapturedObjectPosition(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"capturedObjectsOffset"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capturedObjectsOffset")]
    CapturedObjectsOffset(Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpGravityGunHkParam<'de>, "@name",
    ("grabbedBodies" => GrabbedBodies(Vec<()>)),
    ("maxNumObjectsPicked" => MaxNumObjectsPicked(Primitive<i32>)),
    ("maxMassOfObjectPicked" => MaxMassOfObjectPicked(Primitive<f32>)),
    ("maxDistOfObjectPicked" => MaxDistOfObjectPicked(Primitive<f32>)),
    ("impulseAppliedWhenObjectNotPicked" => ImpulseAppliedWhenObjectNotPicked(Primitive<f32>)),
    ("throwVelocity" => ThrowVelocity(Primitive<f32>)),
    ("capturedObjectPosition" => CapturedObjectPosition(Vector4<f32>)),
    ("capturedObjectsOffset" => CapturedObjectsOffset(Vector4<f32>)),
}
