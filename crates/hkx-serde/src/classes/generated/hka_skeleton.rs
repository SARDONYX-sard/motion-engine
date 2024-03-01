//! A Rust structure that implements a serializer/deserializer corresponding to `hkaSkeleton`, a class defined in C++
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
/// -    size: 84
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaSkeleton<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaSkeleton"`: Name of this class.
    #[serde(default = "HkaSkeleton::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x366e8220`: Unique value of this class.
    #[serde(default = "HkaSkeleton::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaSkeletonHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaSkeletonHkParam<'a>>
}

impl HkaSkeleton<'_> {
    /// Return `"hkaSkeleton"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaSkeleton".into()
    }

    /// Return `"0x366e8220"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x366e8220".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"parentIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "parentIndices")]
    ParentIndices(Vec<i16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"bones"`
    /// -   type: `hkArray&lt;struct hkaBone&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Vec<HkaBone>),
    /// # Information on fields in the original C++ class
    /// -   name:`"referencePose"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referencePose")]
    ReferencePose(Vec<cgmath::Matrix4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"referenceFloats"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceFloats")]
    ReferenceFloats(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatSlots"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatSlots")]
    FloatSlots(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"localFrames"`
    /// -   type: `hkArray&lt;struct hkaSkeletonLocalFrameOnBone&gt;`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrames")]
    LocalFrames(Vec<HkaSkeletonLocalFrameOnBone>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("parentIndices" => ParentIndices(Vec<i16>)),
    ("bones" => Bones(Vec<HkaBone>)),
    ("referencePose" => ReferencePose(Vec<cgmath::Matrix4<f32>>)),
    ("referenceFloats" => ReferenceFloats(Vec<f64>)),
    ("floatSlots" => FloatSlots(Vec<String>)),
    ("localFrames" => LocalFrames(Vec<HkaSkeletonLocalFrameOnBone>)),
}