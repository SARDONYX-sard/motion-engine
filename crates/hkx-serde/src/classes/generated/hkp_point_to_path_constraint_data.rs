//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPointToPathConstraintData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPointToPathConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x8e7cb5da`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPointToPathConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # C++ Class Fields Info
    /// -   name:`"path"`
    /// -   type: `struct hkpParametricCurve*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "path")]
    Path(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"maxFrictionForce"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFrictionForce")]
    MaxFrictionForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angularConstrainedDOF"`
    /// -   type: `enum OrientationConstraintType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularConstrainedDOF")]
    AngularConstrainedDof(Primitive<OrientationConstraintType>),
    /// # C++ Class Fields Info
    /// -   name:`"transform_OS_KS"`
    /// -   type: `hkTransform[2]`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform_OS_KS")]
    TransformOsKs([Transform<f32>; 2]),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPointToPathConstraintData<'de>, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("path" => Path(Cow<'de, str>)),
    ("maxFrictionForce" => MaxFrictionForce(Primitive<f32>)),
    ("angularConstrainedDOF" => AngularConstrainedDof(Primitive<OrientationConstraintType>)),
    ("transform_OS_KS" => TransformOsKs([Transform<f32>; 2])),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrientationConstraintType {
    #[serde(rename = "CONSTRAIN_ORIENTATION_INVALID")]
    ConstrainOrientationInvalid = 0,
    #[serde(rename = "CONSTRAIN_ORIENTATION_NONE")]
    ConstrainOrientationNone = 1,
    #[serde(rename = "CONSTRAIN_ORIENTATION_ALLOW_SPIN")]
    ConstrainOrientationAllowSpin = 2,
    #[serde(rename = "CONSTRAIN_ORIENTATION_TO_PATH")]
    ConstrainOrientationToPath = 3,
    #[serde(rename = "CONSTRAIN_ORIENTATION_MAX_ID")]
    ConstrainOrientationMaxId = 4,
}
