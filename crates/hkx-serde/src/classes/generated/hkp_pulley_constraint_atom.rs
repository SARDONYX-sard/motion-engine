//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPulleyConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpPulleyConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x94a08848`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPulleyConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"fixedPivotAinWorld"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixedPivotAinWorld")]
    FixedPivotAinWorld(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fixedPivotBinWorld"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixedPivotBinWorld")]
    FixedPivotBinWorld(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ropeLength"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ropeLength")]
    RopeLength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"leverageOnBodyB"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leverageOnBodyB")]
    LeverageOnBodyB(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("fixedPivotAinWorld" => FixedPivotAinWorld(Vector4<f32>)),
    ("fixedPivotBinWorld" => FixedPivotBinWorld(Vector4<f32>)),
    ("ropeLength" => RopeLength(Primitive<f32>)),
    ("leverageOnBodyB" => LeverageOnBodyB(Primitive<f32>)),
}
