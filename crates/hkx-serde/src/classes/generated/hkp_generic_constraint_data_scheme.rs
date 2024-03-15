//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGenericConstraintDataScheme`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpGenericConstraintDataScheme`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x11fd6f6c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintDataScheme<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"info"`
    /// -   type: `struct hkpGenericConstraintDataSchemeConstraintInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "info", skip_serializing)]
    Info(HkpGenericConstraintDataSchemeConstraintInfo),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"commands"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "commands")]
    Commands(HkArrayRef<Primitive<i32>>),
    /// # C++ Class Fields Info
    /// -   name:`"modifiers"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "modifiers", skip_serializing)]
    Modifiers(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `hkArray&lt;hkpConstraintMotor*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataScheme<'de>, "@name",
    ("info" => Info(HkpGenericConstraintDataSchemeConstraintInfo)),
    ("data" => Data(HkArrayVector<Vector4<f32>>)),
    ("commands" => Commands(HkArrayRef<Primitive<i32>>)),
    ("modifiers" => Modifiers(HkArrayRef<Cow<'de, str>>)),
    ("motors" => Motors(HkArrayRef<Cow<'de, str>>)),
}
