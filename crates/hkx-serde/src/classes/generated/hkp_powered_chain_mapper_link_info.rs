//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPoweredChainMapperLinkInfo`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPoweredChainMapperLinkInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPoweredChainMapperLinkInfo"`: Name of this class.
    #[serde(default = "HkpPoweredChainMapperLinkInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xcf071a1b`: Unique value of this class.
    #[serde(default = "HkpPoweredChainMapperLinkInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPoweredChainMapperLinkInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPoweredChainMapperLinkInfoHkParam<'a>>
}

impl HkpPoweredChainMapperLinkInfo<'_> {
    /// Return `"hkpPoweredChainMapperLinkInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPoweredChainMapperLinkInfo".into()
    }

    /// Return `"0xcf071a1b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xcf071a1b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainMapperLinkInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"firstTargetIdx"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "firstTargetIdx")]
    FirstTargetIdx(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numTargets"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numTargets")]
    NumTargets(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"limitConstraint"`
    /// -   type: `struct hkpConstraintInstance*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitConstraint")]
    LimitConstraint(Box<HkpConstraintInstance>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperLinkInfoHkParam<'de>, "@name",
    ("firstTargetIdx" => FirstTargetIdx(i32)),
    ("numTargets" => NumTargets(i32)),
    ("limitConstraint" => LimitConstraint(Box<HkpConstraintInstance>)),
}