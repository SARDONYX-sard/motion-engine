//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkMoppBvTreeShapeBase`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMoppBvTreeShapeBase`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpBvTreeShape`/`0xa823d623`
/// - signature: `0x7c338c66`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMoppBvTreeShapeBase<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"code"`
    /// -   type: `struct hkpMoppCode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "code")]
    Code(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"moppData"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "moppData", skip_serializing)]
    MoppData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"moppDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "moppDataSize", skip_serializing)]
    MoppDataSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"codeInfoCopy"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "codeInfoCopy", skip_serializing)]
    CodeInfoCopy(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMoppBvTreeShapeBase<'de>, "@name",
    ("code" => Code(Cow<'de, str>)),
    ("moppData" => MoppData(Cow<'de, str>)),
    ("moppDataSize" => MoppDataSize(Primitive<u32>)),
    ("codeInfoCopy" => CodeInfoCopy(Vector4<f32>)),
}
