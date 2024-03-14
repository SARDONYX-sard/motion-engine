//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpAngLimitConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpAngLimitConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x9be0d9d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAngLimitConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAxis")]
    LimitAxis(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"minAngle"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minAngle")]
    MinAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angularLimitsTauFactor"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularLimitsTauFactor")]
    AngularLimitsTauFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngLimitConstraintAtom, "@name",
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("limitAxis" => LimitAxis(Primitive<u8>)),
    ("minAngle" => MinAngle(Primitive<f32>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(Primitive<f32>)),
}
