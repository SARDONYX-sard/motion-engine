//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBridgeConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBridgeConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x87a4f31b`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBridgeConstraintAtom<'a> {
    /// # C++ Parent class(`hkpConstraintAtom`, parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type", default)]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"buildJacobianFunc"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "buildJacobianFunc", default, skip_serializing)]
    BuildJacobianFunc(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | NOT_OWNED`
    #[serde(rename = "constraintData", default)]
    ConstraintData(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBridgeConstraintAtom<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("buildJacobianFunc" => BuildJacobianFunc(Primitive<Cow<'de, str>>)),
    ("constraintData" => ConstraintData(Primitive<Cow<'de, str>>)),
}
