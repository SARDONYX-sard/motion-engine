//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGenericConstraintDataScheme`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintDataScheme<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"info"`
    /// -   type: `struct hkpGenericConstraintDataSchemeConstraintInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "info", skip_serializing)]
    Info(SingleClass<HkpGenericConstraintDataSchemeConstraintInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayVector<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"commands"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "commands")]
    Commands(HkArrayNum<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"modifiers"`
    /// -   type: `hkArray<void*>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "modifiers", skip_serializing)]
    Modifiers(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `hkArray<hkpConstraintMotor*>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataScheme<'de>, "@name",
    ("info" => Info(SingleClass<HkpGenericConstraintDataSchemeConstraintInfo>)),
    ("data" => Data(HkArrayVector<Vector4<f32>>)),
    ("commands" => Commands(HkArrayNum<i32>)),
    ("modifiers" => Modifiers(HkArrayRef<Cow<'de, str>>)),
    ("motors" => Motors(HkArrayRef<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkpGenericConstraintDataScheme<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
