//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShapeSubpart`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpExtendedMeshShapeSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0xf4608207`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeSubpart<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum SubpartType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<SubpartType>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MaterialIndexStridingType`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MaterialIndexStridingType>),
    /// # C++ Class Fields Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialStriding", skip_serializing)]
    MaterialStriding(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeSubpart<'de>, "@name",
    ("type" => Type(Primitive<SubpartType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MaterialIndexStridingType>)),
    ("materialStriding" => MaterialStriding(Primitive<i16>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
}

impl ByteDeSerialize for HkpExtendedMeshShapeSubpart<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
