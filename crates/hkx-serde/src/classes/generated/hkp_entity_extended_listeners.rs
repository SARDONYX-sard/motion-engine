//! A Rust structure that implements a serializer/deserializer corresponding to `hkpEntityExtendedListeners`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpEntityExtendedListeners<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpEntityExtendedListeners"`: Name of this class.
    #[serde(default = "HkpEntityExtendedListeners::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf557023c`: Unique value of this class.
    #[serde(default = "HkpEntityExtendedListeners::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpEntityExtendedListenersHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpEntityExtendedListenersHkParam<'a>>
}

impl HkpEntityExtendedListeners<'_> {
    /// Return `"hkpEntityExtendedListeners"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpEntityExtendedListeners".into()
    }

    /// Return `"0xf557023c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf557023c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpEntityExtendedListenersHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"activationListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activationListeners", skip_serializing)]
    ActivationListeners(HkpEntitySmallArraySerializeOverrideType),
    /// # Information on fields in the original C++ class
    /// -   name:`"entityListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "entityListeners", skip_serializing)]
    EntityListeners(HkpEntitySmallArraySerializeOverrideType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntityExtendedListenersHkParam<'de>, "@name",
    ("activationListeners" => ActivationListeners(HkpEntitySmallArraySerializeOverrideType)),
    ("entityListeners" => EntityListeners(HkpEntitySmallArraySerializeOverrideType)),
}