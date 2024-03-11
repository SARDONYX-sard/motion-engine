//! A Rust structure that implements a serializer/deserializer corresponding to `hkbRigidBodyRagdollControlData`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbRigidBodyRagdollControlData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbRigidBodyRagdollControlData"`: The original C++ class name.
    #[serde(default = "HkbRigidBodyRagdollControlData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1e0bc068`: Unique value of this class.
    #[serde(default = "HkbRigidBodyRagdollControlData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbRigidBodyRagdollControlDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbRigidBodyRagdollControlDataHkParam<'a>>
}

impl HkbRigidBodyRagdollControlData<'_> {
    /// Return `"hkbRigidBodyRagdollControlData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbRigidBodyRagdollControlData".into()
    }

    /// Return `"0x1e0bc068"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1e0bc068".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRigidBodyRagdollControlDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"keyFrameHierarchyControlData"`
    /// -   type: `struct hkaKeyFrameHierarchyUtilityControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "keyFrameHierarchyControlData")]
    KeyFrameHierarchyControlData(HkaKeyFrameHierarchyUtilityControlData),
    /// # Field information in the original C++ class
    /// -   name:`"durationToBlend"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "durationToBlend")]
    DurationToBlend(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbRigidBodyRagdollControlDataHkParam<'de>, "@name",
    ("keyFrameHierarchyControlData" => KeyFrameHierarchyControlData(HkaKeyFrameHierarchyUtilityControlData)),
    ("durationToBlend" => DurationToBlend(Primitive<f32>)),
}
